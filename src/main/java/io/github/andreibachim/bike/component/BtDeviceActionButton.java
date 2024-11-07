package io.github.andreibachim.bike.component;

import io.github.andreibachim.bike.bluetooth.BtDevice;
import io.github.jwharm.javagi.gobject.annotations.RegisteredType;
import lombok.extern.slf4j.Slf4j;
import org.gnome.adw.ActionRow;
import org.gnome.adw.Bin;
import org.gnome.adw.Spinner;
import org.gnome.gtk.*;

import java.lang.foreign.MemorySegment;

@Slf4j
@RegisteredType(name = "BtDeviceActionButton")
public class BtDeviceActionButton extends ActionRow {

    private String status = "Not set up";
    private Bin statusContainer = new Bin();
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
        onActivate(() -> statusContainer.setChild(Spinner.builder().build()));
        addController(GestureClick.builder().onReleased((_, _, _) -> emitActivate()).build());
        setChild(buildUI());
    }

    private Widget buildUI() {
        Box container = Box.builder()
                .setOrientation(Orientation.HORIZONTAL)
                .setCssClasses(new String[]{"header"})
                .setHexpand(true)
                .build();
        Label title = Label.builder().setLabel(device.getName()).setHexpand(true).setHalign(Align.START).build();
        Label statusLabel = Label.builder()
                .setLabel(status)
                .setCssClasses(new String[]{"dim-label"})
                .setHexpand(false)
                .setHalign(Align.END).build();
        container.append(title);
        statusContainer.setChild(statusLabel);
        container.append(statusContainer);
        return container;
    }
}
