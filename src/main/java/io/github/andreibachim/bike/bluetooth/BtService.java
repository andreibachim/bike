package io.github.andreibachim.bike.bluetooth;

import java.util.Arrays;
import java.util.LinkedHashMap;
import java.util.Map;
import java.util.Objects;
import java.util.Optional;
import java.util.concurrent.Executors;
import java.util.concurrent.TimeUnit;
import java.util.stream.Stream;

import org.bluez.Device1;
import org.freedesktop.dbus.exceptions.DBusException;
import org.freedesktop.dbus.handlers.AbstractInterfacesAddedHandler;
import org.freedesktop.dbus.handlers.AbstractInterfacesRemovedHandler;
import org.freedesktop.dbus.handlers.AbstractPropertiesChangedHandler;
import org.freedesktop.dbus.interfaces.ObjectManager;
import org.freedesktop.dbus.interfaces.Properties;
import org.freedesktop.dbus.interfaces.Properties.PropertiesChanged;
import org.freedesktop.dbus.types.Variant;

import com.github.hypfvieh.DbusHelper;
import com.github.hypfvieh.bluetooth.DeviceManager;
import com.github.hypfvieh.bluetooth.DiscoveryFilter;
import com.github.hypfvieh.bluetooth.DiscoveryTransport;
import com.github.hypfvieh.bluetooth.wrapper.AgentManager;
import com.github.hypfvieh.bluetooth.wrapper.BluetoothAdapter;
import com.github.hypfvieh.bluetooth.wrapper.BluetoothGattService;

import io.github.andreibachim.bike.constant.UUIDs;
import lombok.Getter;
import lombok.Setter;
import lombok.extern.slf4j.Slf4j;

@Slf4j
public class BtService {
  private BtService() {
  }

  @Getter
  private static final Optional<BtService> instance = BtService.create();

  @Getter
  private Optional<BtDevice> device = Optional.empty();

  private DeviceManager deviceManager;
  private BluetoothAdapter adapter;

  private static Optional<BtService> create() {
    BtService btService = new BtService();
    try {
      btService.deviceManager = DeviceManager.createInstance(false);
      new AgentManager(btService.deviceManager.getDbusConnection()).registerAgent("/io/github/andreibachim/bike",
          "NoInputNoOutput");
      btService.adapter = btService.deviceManager.getAdapter();
      if (Objects.isNull(btService.adapter)) {
        log.error("No adapter found");
        return Optional.empty();
      }

      // Set the bt device
      btService.setDevice(btService.getKnownFitnessMachines().filter(BtDevice::isConnected).findFirst());

      Map<DiscoveryFilter, Object> filters = new LinkedHashMap<>();
      filters.put(DiscoveryFilter.Transport, DiscoveryTransport.LE);
      String[] relevantServices = new String[] { UUIDs.FITNESS_MACHINE_SERVICE, UUIDs.CYCLING_POWER_SERVICE };
      filters.put(DiscoveryFilter.UUIDs, relevantServices);
      btService.deviceManager.setScanFilter(filters);

    } catch (DBusException e) {
      log.error("Could not create bluetooth device manager", e);
      return Optional.empty();
    } catch (Exception e) {
      log.error("Unexpected error occurred when creating the bluetooth service", e);
      return Optional.empty();
    }
    return Optional.of(btService);
  }

  public boolean isAdapterOn() {
    return adapter.isPowered();
  }

  public boolean startDiscovery() {
    return adapter.startDiscovery();
  }

  public boolean stopDiscovery() {
    return adapter.stopDiscovery();
  }

  public void registerAdapterStateListener(AdapterStateListener listener) {
    try {
      deviceManager.registerPropertyHandler(new AbstractPropertiesChangedHandler() {
        @Override
        public void handle(Properties.PropertiesChanged signal) {
          final Variant<?> poweredVariant = signal.getPropertiesChanged().get("Powered");
          if (Objects.nonNull(poweredVariant)) {
            boolean powered = (boolean) poweredVariant.getValue();
            if (powered)
              listener.powerOn();
            else
              listener.powerOff();
          }
        }
      });
    } catch (Exception e) {
      log.error("Could not register adapter listener");
    }
  }

  public interface AdapterStateListener {
    void powerOn();

    void powerOff();
  }

  public void registerDeviceAvailabilityListener(DeviceAvailabilityListener listener) throws DBusException {
    deviceManager.getDevices(true)
        .stream()
        .filter(device -> Stream
            .of(device.getUuids())
            .anyMatch(uuid -> uuid.equalsIgnoreCase(UUIDs.FITNESS_MACHINE_SERVICE)))
        .map(BtDevice::new)
        .forEach(listener::deviceFound);
    deviceManager.registerSignalHandler(new AbstractInterfacesAddedHandler() {
      @Override
      public void handle(ObjectManager.InterfacesAdded signal) {
        log.info(signal.toString());
        listener.deviceFound(signalToDevice(signal.getSignalSource().toString()));
      }
    });
    deviceManager.registerSignalHandler(new AbstractInterfacesRemovedHandler() {
      @Override
      public void handle(ObjectManager.InterfacesRemoved signal) {
        listener.deviceLost(signalToDevice(signal.getSignalSource().toString()).getAddress());
      }
    });
  }

  public interface DeviceAvailabilityListener {
    void deviceFound(BtDevice device);

    void deviceLost(String address);
  }

  public void registerDeviceStatusListener(DeviceStatusListener listener) throws DBusException {
    deviceManager.registerPropertyHandler(new AbstractPropertiesChangedHandler() {
      public void handle(PropertiesChanged signal) {
        final Variant<?> connectedVariant = signal.getPropertiesChanged().get("Connected");
        if (Objects.nonNull(connectedVariant)) {
          boolean connected = (boolean) connectedVariant.getValue();
          BtDevice device = signalToDevice(signal.getPath());
          if (connected) {
            listener.deviceConnected(device);
            setDevice(Optional.of(device));
          } else {
            listener.deviceDisconnected(device);
          }
        }
      }
    });
  }

  public interface DeviceStatusListener {
    void deviceConnected(BtDevice device);

    void deviceDisconnected(BtDevice device);
  }

  public void setDevice(Optional<BtDevice> device) {
    this.device = device;
    device.ifPresent(d -> {

      // log.info("Is device paired? {}", d.isPaired());
      // log.info("Is device trusted? {}", d.isTrusted());
      // log.info("Is device blocked? {}", d.isBlocked());
      // log.info("Is device connected? {}", d.isConnected());
      // log.info("Are services discovered? {}", d.isServicesResolved());
      //
      // d.refreshGattServices();
      BluetoothGattService gattService = d.getGattServiceByUuid(UUIDs.FITNESS_MACHINE_SERVICE);
      gattService.refreshGattCharacteristics();
      for (var characteristic : gattService.getGattCharacteristics()) {
        log.info("Printing values of characteristic {}", characteristic.getUuid());
        log.info("{}", characteristic.getValue().length);
      }
    });
  }

  private BtDevice signalToDevice(String path) {
    Device1 rawDevice = DbusHelper.getRemoteObject(deviceManager.getDbusConnection(), path, Device1.class);
    return new BtDevice(rawDevice, adapter, path, deviceManager.getDbusConnection());
  }

  private Stream<BtDevice> getKnownFitnessMachines() {
    return deviceManager.getDevices(true)
        .stream()
        .filter(device -> device.getGattServices().stream().map(BluetoothGattService::getUuid).toList()
            .containsAll(Arrays.asList(UUIDs.FITNESS_MACHINE_SERVICE, UUIDs.CYCLING_POWER_SERVICE)))
        .map(BtDevice::new);
  }
}
