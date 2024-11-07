package io.github.andreibachim.bike.component;

import io.github.andreibachim.bike.bluetooth.BtDevice;
import io.github.andreibachim.bike.bluetooth.BtService;
import io.github.jwharm.javagi.gobject.annotations.InstanceInit;
import io.github.jwharm.javagi.gobject.annotations.RegisteredType;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.gnome.adw.*;
import org.gnome.glib.GLib;
import org.gnome.gtk.Button;
import org.gnome.gtk.GtkBuilder;

import java.lang.foreign.MemorySegment;

import static io.github.andreibachim.bike.component.BtIndicator.Status.*;

@Slf4j
@RegisteredType(name = "BtIndicator")
public class BtIndicator extends Button {
    public BtIndicator(MemorySegment address) {
        super(address);
    }

    private Status status;

    private void setStatus(Status status) {
        this.status = status;
        setIconName(status.getIconName());
    }

    @InstanceInit
    public void init() {
        BtService.getInstance().ifPresentOrElse(btService -> {
            if (btService.isAdapterOn()) setStatus(DISCONNECTED);
            else setStatus(DISABLED);
            btService.registerAdapterStateListener(new BtService.AdapterStateListener() {
                @Override
                public void powerOn() {
                    setStatus(DISCONNECTED);
                }

                @Override
                public void powerOff() {
                    setStatus(DISABLED);
                }
            });
            onClicked(() -> {
                switch (status) {
                    case NO_HARDWARE -> {
                        AlertDialog dialog = new AlertDialog("No Bluetooth", "A bluetooth adapter is required to use this application.");
                        dialog.addResponse("ok", "Ok");
                        dialog.present(getAncestor(ApplicationWindow.getType()));
                    }
                    case DISABLED -> {
                        AlertDialog dialog = new AlertDialog("Bluetooth disabled", "Bluetooth is turned off on this device.");
                        dialog.addResponse("ok", "Ok");
                        dialog.present(getAncestor(ApplicationWindow.getType()));
                    }
                    case DISCONNECTED -> disconnectedHandler(btService);
                    case ACTIVE -> {
                        //TODO Implemenet disconnect dialog
                    }
                }
            });
        }, () -> setStatus(NO_HARDWARE));
    }

    private void disconnectedHandler(BtService service) {
        GtkBuilder builder = GtkBuilder
                .fromResource("/io/github/andreibachim/bike/ui/components/dialogs/connect.ui");
        final Dialog dialog = (Dialog) builder.getObject("connect");
        dialog.present(getAncestor(ApplicationWindow.getType()));
        PreferencesGroup list = ((PreferencesGroup) builder.getObject("list"));
        try {
            service.registerDeviceStatusListener(new BtService.DeviceStatusListener() {
                @Override
                public void deviceFound(BtDevice device) {
                    list.add(new BtDeviceActionButton(device));
                }

                @Override
                public void deviceLost(String address) {
                    //TODO implement logic of removing device
                }
            });
        } catch (Exception e) {
            //TODO Close the window. Currently not possible to implement because this is thrown before the dialog is presented
            ToastOverlay toastOverlay = (ToastOverlay) getAncestor(ToastOverlay.getType());
            toastOverlay.addToast(new Toast("Could not search for new devices"));
        }
    }

    @Getter
    @RequiredArgsConstructor
    enum Status {
        NO_HARDWARE("bluetooth-hardware-disabled"),
        DISABLED("bluetooth-disabled"),
        DISCONNECTED("bluetooth-disconnected"),
        ACTIVE("bluetooth-active");
        final String iconName;
    }
}
