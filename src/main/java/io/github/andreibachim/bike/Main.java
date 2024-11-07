package io.github.andreibachim.bike;

import io.github.andreibachim.bike.component.BtDeviceActionButton;
import io.github.andreibachim.bike.component.BtIndicator;
import io.github.jwharm.javagi.gtk.types.TemplateTypes;
import lombok.extern.slf4j.Slf4j;
import org.gnome.adw.Application;
import org.gnome.gio.Resource;

import io.github.andreibachim.bike.ui.Window;
import io.github.jwharm.javagi.base.GErrorException;

@Slf4j
public class Main {
    public static void main(String[] args) {
        try {
            new Main(args);
        } catch (GErrorException e) {
            log.error("Could not start the application", e);
            System.exit(e.getCode());
        }
    }

    public Main(String[] args) throws GErrorException {
        loadResources();
        loadCustomTypes();

        final Application app = new Application("io.github.andreibachim.bike");
        app.onActivate(() -> buildUI(app));
        app.run(args);

    }

    private void buildUI(Application app) {
        final Window window = Window.create(app);
        window.present();
    }

    private void loadResources() throws GErrorException {
        Resource resource = Resource.load("src/main/resources/resources.gresource");
        resource.resourcesRegister();
    }

    private void loadCustomTypes() {
        TemplateTypes.register(BtIndicator.class);
        TemplateTypes.register(BtDeviceActionButton.class);
    }
}
