package io.github.andreibachim.bike.ui;

import java.lang.foreign.MemorySegment;

import org.gnome.adw.Application;
import org.gnome.adw.ApplicationWindow;
import org.gnome.glib.Type;
import org.gnome.gobject.GObject;

import io.github.jwharm.javagi.gtk.annotations.GtkTemplate;
import io.github.jwharm.javagi.gtk.types.TemplateTypes;

@GtkTemplate(name = "Window", ui = "/io/github/andreibachim/bike/ui/window.ui")
public class Window extends ApplicationWindow {
    public static final Type gtype = TemplateTypes.register(Window.class);

    public Window(MemorySegment address) {
        super(address);
    }

    public static Window create(Application app) {
        Window instance = GObject.newInstance(gtype);
        instance.asParent().setApplication(app);
        return instance;
    }
}
