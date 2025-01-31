use ratatui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, Table},
};

use crate::structs::Characteristic;

/// Provides an overlay with the selected device's services.
pub fn inspect_overlay(
    characteristics: &[Characteristic],
    scroll: usize,
    height: u16,
) -> Table<'static> {
    let mut rows: Vec<Row> = Vec::new();

    for characteristic in characteristics.iter() {
        let service_uuid = characteristic.service.to_string();
        rows.push(
            Row::new(vec![format!("Service: {service_uuid}")])
                .style(Style::default().fg(Color::Gray)),
        );

        let properties = format!(
            "{:?}",
            characteristic
                .properties
                .iter_names()
                .map(|x| x.0.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );

        rows.push(Row::new(vec![format!(
            "--> {} ({})",
            characteristic.uuid.to_string(),
            properties
        )]));

        for descriptor in characteristic.descriptors.iter() {
            let descriptor_row = Row::new(vec![format!(
                "    |-- Descriptor: {}",
                descriptor.to_string()
            )]);
            rows.push(descriptor_row);
        }
    }

    let adjusted_height = if height > 3 { height - 3 } else { height };
    let visible_rows_count = adjusted_height as usize;

    let total_rows = rows.len();
    let start_index = scroll;
    let end_index = usize::min(start_index + visible_rows_count, total_rows);

    let visible_rows = if start_index < total_rows {
        &rows[start_index..end_index]
    } else {
        &[]
    };

    Table::new(visible_rows.to_vec(), [Constraint::Percentage(100)])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Characteristics")
                .border_style(Style::default().fg(Color::Yellow)),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
}
