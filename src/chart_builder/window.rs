
use chart_builder::*;


/*
 * enum declaration for determining chart type.
 */

#[derive(Clone)]
pub(in chart_builder) enum ChartType {
    Hist(Histogram),
    BoxWhisk(BoxWhiskerPlot),
    Doughnut(DoughnutChart),
    Pie(PieChart),
    VBar(VerticalBarChart),
    Radar(RadarChart),
    Area(AreaChart),
    StackedArea(StackedAreaChart),
    Line(LineChart),
    XYScat(XYScatterPlot),
    Bubble(BubbleChart),
}

/*
 * Window creation functions - adapted from gtk-rs/examples
 */

// make moving clones into closures more convenient
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

fn window_setup(chart_prop: ChartProp, window: &gtk::ApplicationWindow) {
    window.set_title(chart_prop.chart_title.as_str());
    window.set_default_size(
        if chart_prop.show_legend == false { chart_prop.screen_size.0 as i32 } else {(chart_prop.screen_size.0 * 1.30).ceil() as i32}
        , chart_prop.screen_size.1 as i32);
    //window.set_resizable(false); - appears to not work
}

// Call draw_chart method in the specified chart structure
fn build_ui(application: &gtk::Application, chart_type: ChartType) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_position(gtk::WindowPosition::Center);
    let drawing_area = Box::new(DrawingArea::new)();

    match chart_type {
        ChartType::Hist(c) => { window_setup(c.get_chart_prop(), &window);
                                c.draw_chart(&drawing_area); },
        ChartType::BoxWhisk(c) => { window_setup(c.get_chart_prop(), &window);
                                c.draw_chart(&drawing_area); },
        ChartType::Doughnut(c) => { window_setup(c.get_chart_prop(), &window);
                                c.draw_chart(&drawing_area); },
        ChartType::Pie(c) => { window_setup(c.get_chart_prop(), &window);
                                c.draw_chart(&drawing_area); },
        ChartType::VBar(c) => { window_setup(c.get_chart_prop(), &window);
                                c.draw_chart(&drawing_area); },
        ChartType::Radar(c) => { window_setup(c.get_chart_prop(), &window);
                                c.draw_chart(&drawing_area); },
        ChartType::Area(c) => { window_setup(c.get_chart_prop(), &window);
                                c.draw_chart(&drawing_area); },
        ChartType::StackedArea(c) => { window_setup(c.get_chart_prop(), &window);
                                c.draw_chart(&drawing_area); },
        ChartType::Line(c) => { window_setup(c.get_chart_prop(), &window);
                                c.draw_chart(&drawing_area); },
        ChartType::XYScat(c) => { window_setup(c.get_chart_prop(), &window);
                                c.draw_chart(&drawing_area); },
        ChartType::Bubble(c) => { window_setup(c.get_chart_prop(), &window);
                                c.draw_chart(&drawing_area); },
    };

    window.connect_delete_event(clone!(window => move |_, _| {
        window.destroy();
        Inhibit(false)
    }));
    window.add(&drawing_area);
    window.show_all();
}

// Create GUI window and call Cairo drawing function
pub(in chart_builder) fn build_window(chart_type: ChartType) {
    let application = gtk::Application::new("com.github.rustlib_app",
                                            gio::ApplicationFlags::empty())
                                       .expect("Initialization failed...");

    application.connect_startup(move |app| {
        build_ui(app, chart_type.clone());
    });
    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}
