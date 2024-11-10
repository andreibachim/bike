package io.github.andreibachim.bike.component;

import java.lang.foreign.MemorySegment;
import java.util.concurrent.Executors;

import org.gnome.adw.ActionRow;
import org.gnome.adw.Bin;
import org.gnome.adw.Dialog;
import org.gnome.adw.Spinner;
import org.gnome.adw.Toast;
import org.gnome.adw.ToastOverlay;
import org.gnome.glib.GLib;
import org.gnome.gtk.Align;
import org.gnome.gtk.Box;
import org.gnome.gtk.GestureClick;
import org.gnome.gtk.Label;
import org.gnome.gtk.Orientation;
import org.gnome.gtk.Widget;

import io.github.andreibachim.bike.bluetooth.BtDevice;
import io.github.jwharm.javagi.gobject.annotations.RegisteredType;
import lombok.extern.slf4j.Slf4j;

@Slf4j
@RegisteredType(name = "BtDeviceActionButton")
public class BtDeviceActionButton extends ActionRow {

  private final Bin statusContainer = new Bin();
  private final Label statusLabel = (Label) Label.builder()
        .setLabel("Not set up")
        .setCssClasses(new String[] { "dim-label" })
        .setHexpand(false)
        .setHalign(Align.END)
        .build();  
  private BtDevice device;

  public BtDeviceActionButton(MemorySegment address) {
    super(address);
  }

  public BtDeviceActionButton(BtDevice device) {
    super();
    this.device = device;
    if (Boolean.TRUE.equals(device.isPaired())) {
      statusLabel.setLabel("Disconnected");
    }
    if (Boolean.TRUE.equals(device.isConnected())) {
      statusLabel.setLabel("Connected");
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
            ((ToastOverlay) getAncestor(ToastOverlay.getType()))
                .addToast(Toast.builder().setTitle("Could not pair").build());
          }
        }
        if (Boolean.FALSE.equals(device.isConnected())) {
          if(device.connect()) {
            GLib.idleAddOnce(() -> {
              statusLabel.setLabel("Connected");
              statusContainer.setChild(statusLabel);
              ((Dialog) getAncestor(Dialog.getType())).close();
            });
          }
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
    container.append(title);
    statusContainer.setChild(statusLabel);
    container.append(statusContainer);
    return container;
  }
}
