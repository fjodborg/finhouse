use super::entry::{Entry, MultiLines, ParameterWidget};
use super::finhouse;
use egui::{Label, Ui, Widget};

pub trait SideBar {
    fn create_side_panel(&mut self, ui: &mut Ui);
}

impl SideBar for finhouse::FinhouseApp {
    fn create_side_panel(&mut self, ui: &mut Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("side_bar")
                .num_columns(2)
                .spacing([20.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    let entry = &mut self.entries[self.selected_entry];
                    sidebar_content(ui, entry);
                });

            ui.separator();
            egui::warn_if_debug_build(ui);
            source_code(ui);
        });
    }
}

fn sidebar_widget(ui: &mut Ui, label: &str, widget: impl Widget) {
    ui.add(Label::new(label));
    ui.add(widget);
    ui.end_row();
}

fn sidebar_multi_widget(ui: &mut Ui, multi_widgets: Vec<(impl Widget, impl Widget)>) {
    for widgets in multi_widgets {
        let (label, value) = widgets;
        ui.add(label);
        ui.add(value);
        ui.end_row();
    }
}

fn sidebar_content(ui: &mut Ui, entry: &mut Entry) {
    ui.heading("Bolig:");
    ui.end_row();
    sidebar_widget(ui, "Tab Navn", entry.name_widget());
    sidebar_widget(ui, "Bolig pris", entry.house_price_widget());
    sidebar_widget(ui, "Egen betaling", entry.initial_payment_widget());

    // TODO: Add inflation
    // TODO: Readd this, once properly implemented.
    // sidebar_widget(ui, "Årlig værdi stigning", entry.value_increase_widget());

    ui.label("");
    ui.end_row();
    ui.heading("Rente:");
    ui.end_row();

    sidebar_widget(ui, "Rente [ÅOP]", entry.interest_widget());
    sidebar_widget(ui, "Rentefradrag", entry.interest_deduction_widget());
    sidebar_widget(ui, "Låneperiode", entry.payment_duration_widget());
    sidebar_widget(
        ui,
        "Ydelse før fradrag",
        entry.monthly_payment_widget(false),
    );
    sidebar_widget(
        ui,
        "Ydelse efter fradrag",
        entry.monthly_payment_widget(true),
    );

    // TODO: find a proper solution to get stripes to work.
    ui.label("");
    ui.end_row();
    ui.heading("Diverse");
    ui.end_row();
    sidebar_widget(ui, "Indkomst efter skat", entry.income_widget());
    sidebar_widget(
        ui,
        "Rådighedsbeløb efter skat",
        entry.available_amount_widget(),
    );
    sidebar_widget(ui, "Formue + værdi", egui::Label::new("TODO"));
    sidebar_widget(ui, "Penge betalt til rente", entry.money_paid_widget(true));
    sidebar_widget(ui, "Penge betalt for bolig", entry.money_paid_widget(false));
    sidebar_widget(
        ui,
        "Penge betalt for alt",
        entry.money_paid_and_expenses_widget(),
    );

    // sidebar_widget(ui, "", egui::Label::new("TODO"));

    ui.label("");
    ui.end_row();

    // TODO: Readd this, once loan stop is properly implemented.
    // ui.heading("Aktier:");
    // ui.end_row();
    // sidebar_widget(ui, "Investerings værdi", entry.investments_widget());
    // sidebar_widget(ui, "Forventet afkast", entry.investments_gain_widget());
    // sidebar_widget(ui, "Aktie skat", entry.investments_tax_widget());
    // ui.end_row();

    ui.heading("Månedlige Udgifter:");
    ui.end_row();
    sidebar_multi_widget(ui, entry.monthly_expenses_widget());

    ui.label("");
    ui.end_row();
    if ui.button("Ryd udgifter!").highlight().clicked() {
        // new_entry = true;
        entry.monthly_expenses.clear();
    }
    if ui.button("Ny udgift!").highlight().clicked() {
        // new_entry = true;
        entry.monthly_expenses.push(MultiLines {
            name: "Månedlig udgift".into(),
            value: 0,
        });
    }
}

fn source_code(ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Source code: ");
        ui.hyperlink_to("github", "https://github.com/fjodborg/finhouse");
    });
}
