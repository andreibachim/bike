package io.github.andreibachim.bike.component;

import io.github.andreibachim.bike.bluetooth.BtDevice;
import io.github.jwharm.javagi.gobject.annotations.RegisteredType;
import lombok.extern.slf4j.Slf4j;
import org.gnome.adw.ActionRow;
import org.gnome.adw.Bin;
import org.gnome.adw.Spinner;
import org.gnome.adw.Toast;
import org.gnome.adw.ToastOverlay;
import org.gnome.glib.GLib;
import org.gnome.gtk.*;

import java.lang.foreign.MemorySegment;
import java.util.concurrent.Executors;

@Slf4j
@RegisteredType(name = "BtDeviceActionButton")
public class BtDeviceActionButton extends ActionRow {

  private String status = "Not set up";
  private final Bin statusContainer = new Bin();
  private BtDevice device;

  public BtDeviceActionButton(MemorySegment address) {
    super(address);
  }

  public BtDeviceActionButton(BtDevice device) {
    super();
    this.device = device;
    if (Boolean.TRUE.equals(device.isPaired())) {
      status = "Disconnected";
    }
    if (Boolean.TRUE.equals(device.isConnected())) {
      status = "Connected";
    }
    setActivatable(true);
    onActivate(() -> {
      statusContainer.setChild(Spinner.builder().build());
      Executors.newSingleThreadExecutor().execute(() -> {
        if (Boolean.FALSE.equals(device.isTrusted())) {
          device.setTrusted(true);
        }

        if (Boolean.FALSE.equals(device.isPaired())) {
          if (device.pair())
            ;
          else {
            ((ToastOverlay) getAncestor(ToastOverlay.getType())).addToast(Toast.builder().setTitle("Could not pair").build());
          }
        }

        if (Boolean.FALSE.equals(device.isConnected())) {
          device.connect();
        }
      });
    });
    addController(GestureClick.builder().onReleased((_, _, _) -> emitActivate()).build());
    setChild(buildUI());
  }

  private Widget buildUI() {
    Box container = (Box) Box.builder()
        .setOrientation(Orientation.HORIZONTAL)
        .setCssClasses(new String[] { "header" })
        .setHexpand(true)
        .build();
    Label title = (Label) Label.builder().setLabel(device.getName()).setHexpand(true).setHalign(Align.START).build();
    Label statusLabel = (Label) Label.builder()
        .setLabel(status)
        .setCssClasses(new String[] { "dim-label" })
        .setHexpand(false)
        .setHalign(Align.END).build();
    container.append(title);
    statusContainer.setChild(statusLabel);
    container.append(statusContainer);
    return container;
  }
}
