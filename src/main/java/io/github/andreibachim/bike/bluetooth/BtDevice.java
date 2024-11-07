package io.github.andreibachim.bike.bluetooth;

import com.github.hypfvieh.bluetooth.wrapper.BluetoothAdapter;
import com.github.hypfvieh.bluetooth.wrapper.BluetoothDevice;
import org.bluez.Device1;
import org.freedesktop.dbus.connections.impl.DBusConnection;

public class BtDevice extends BluetoothDevice {

    public BtDevice(Device1 device, BluetoothAdapter adapter, String dbusPath, DBusConnection dbusConnection) {
        super(device, adapter, dbusPath, dbusConnection);
    }

    public BtDevice(BluetoothDevice device) {
        super(device.getRawDevice(), device.getAdapter(), device.getDbusPath(), device.getDbusConnection());
    }
}
