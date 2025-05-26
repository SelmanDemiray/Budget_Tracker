use crate::models::{Budget, Category, Transaction, TransactionType};
use chrono::{Datelike, Duration};
use iced::{Element, Length, Alignment};
use iced::widget::{column, text, container};
use plotters::prelude::*;
use plotters_iced::ChartWidget;
use std::collections::HashMap;

const CHART_HEIGHT: u16 = 300;
const CHART_WIDTH: u16 = 600;

pub fn create_expense_by_category_chart(
    transactions: &[Transaction],
    categories: &[Category],
) -> Element<'static, crate::ui::Message> {
    let mut category_totals: HashMap<String, f64> = HashMap::new();
    let category_map: HashMap<String, &Category> = categories
        .iter()
        .map(|c| (c.id.clone(), c))
        .collect();

    // Sum transactions by category
    for transaction in transactions.iter() {
        if transaction.is_expense() {
            if let Some(cat_id) = &transaction.category_id {
                *category_totals.entry(cat_id.clone()).or_insert(0.0) += transaction.amount;
            } else {
                *category_totals.entry("uncategorized".to_string()).or_insert(0.0) += transaction.amount;
            }
        }
    }

    // Sort categories by amount (descending)
    let mut categories_sorted: Vec<(String, f64)> = category_totals.into_iter().collect();
    categories_sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    let chart = ChartWidget::new(move |_, area| {
        let backend = plotters_iced::DrawingBackend::new(area);
        let root = backend.into_drawing_area();
        root.fill(&WHITE)?;

        let total: f64 = categories_sorted.iter().map(|c| c.1).sum();
        if total == 0.0 {
            let style = TextStyle::from(("sans-serif", 20).into_font()).color(&BLACK);
            root.draw_text("No expense data available", &style, (150, 150))?;
            return Ok(());
        }

        let mut chart = ChartBuilder::on(&root)
            .caption("Expenses by Category", ("sans-serif", 20))
            .build_cartesian_2d(0.0..1.0, 0.0..1.0)?;

        let mut start_angle = 0.0;
        for (i, (cat_id, amount)) in categories_sorted.iter().enumerate().take(5) {
            let percentage = amount / total;
            let end_angle = start_angle + percentage * 360.0;
            
            let category_name = match category_map.get(cat_id) {
                Some(cat) => cat.name.clone(),
                None => if cat_id == "uncategorized" { "Uncategorized".to_string() } else { "Unknown".to_string() }
            };
            
            let color_hex = match category_map.get(cat_id) {
                Some(cat) => cat.color.clone(),
                None => "#CCCCCC".to_string()
            };
            
            let color = parse_color(&color_hex);
            
            // Draw pie slice
            chart.draw_series(std::iter::once(Pie::new(
                (0.5, 0.5),
                0.4,
                (start_angle, end_angle),
                color.filled(),
            )))?;
            
            // Draw label
            let angle = (start_angle + end_angle) / 2.0 * std::f64::consts::PI / 180.0;
            let x = 0.5 + 0.45 * angle.cos();
            let y = 0.5 + 0.45 * angle.sin();
            
            let label = format!("{}: ${:.2} ({:.1}%)", category_name, amount, percentage * 100.0);
            
            root.draw_text(
                &label,
                &TextStyle::from(("sans-serif", 12).into_font()).color(&BLACK),
                (
                    (x * area.width() as f64) as i32,
                    (y * area.height() as f64) as i32,
                ),
            )?;
            
            start_angle = end_angle;
        }

        Ok(())
    });

    Column::new()
        .push(
            Text::new("Expenses by Category")
                .size(20)
                .style(Text::Default)
        )
        .push(
            Container::new(chart.height(Length::Fixed(CHART_HEIGHT as f32)))
                .width(Length::Fixed(CHART_WIDTH as f32))
                .style(Container::Box)
        )
        .spacing(10)
        .align_items(Alignment::Center)
        .into()
}

pub fn create_income_vs_expense_chart(
    transactions: &[Transaction],
) -> Element<'static, crate::ui::Message> {
    // Group transactions by month
    let mut monthly_data: HashMap<(i32, u32), (f64, f64)> = HashMap::new(); // (year, month) -> (income, expenses)
    
    let now = chrono::Local::now().naive_local().date();
    let six_months_ago = now - Duration::days(180);
    
    // Initialize past 6 months
    for i in 0..6 {
        let date = six_months_ago + Duration::days(30 * i);
        monthly_data.insert((date.year(), date.month()), (0.0, 0.0));
    }
    
    // Sum transactions by month
    for transaction in transactions.iter() {
        if transaction.date >= six_months_ago {
            let key = (transaction.date.year(), transaction.date.month());
            let entry = monthly_data.entry(key).or_insert((0.0, 0.0));
            
            match transaction.transaction_type {
                TransactionType::Income => entry.0 += transaction.amount,
                TransactionType::Expense => entry.1 += transaction.amount,
            }
        }
    }
    
    // Sort months
    let mut monthly_data_sorted: Vec<((i32, u32), (f64, f64))> = monthly_data.into_iter().collect();
    monthly_data_sorted.sort_by_key(|((year, month), _)| (*year * 100 + *month as i32));
    
    let chart = ChartWidget::new(move |_, area| {
        let backend = plotters_iced::DrawingBackend::new(area);
        let root = backend.into_drawing_area();
        root.fill(&WHITE)?;
        
        let max_amount = monthly_data_sorted.iter()
            .flat_map(|(_, (income, expenses))| vec![*income, *expenses])
            .fold(0.0, |acc, val| if val > acc { val } else { acc });
            
        let x_labels: Vec<String> = monthly_data_sorted.iter()
            .map(|((year, month), _)| format!("{}/{}", month, year % 100))
            .collect();
            
        let mut chart = ChartBuilder::on(&root)
            .caption("Income vs Expenses", ("sans-serif", 20))
            .margin(5)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(0..monthly_data_sorted.len(), 0.0..max_amount * 1.1)?;
            
        chart.configure_mesh()
            .x_labels(monthly_data_sorted.len())
            .x_label_formatter(&|idx| {
                if *idx < x_labels.len() {
                    x_labels[*idx].clone()
                } else {
                    "".to_string()
                }
            })
            .y_label_formatter(&|y| format!("${:.0}", y))
            .draw()?;
            
        // Draw income bars
        chart.draw_series(
            monthly_data_sorted.iter().enumerate().map(|(i, (_, (income, _)))| {
                let bar_width = 0.3;
                Rectangle::new(
                    [(i as f32 - bar_width as f32 / 2.0) as i32, 0.0],
                    [(i as f32 + bar_width as f32 / 2.0) as i32, *income],
                    GREEN.filled(),
                )
            })
        )?;
        
        // Draw expense bars
        chart.draw_series(
            monthly_data_sorted.iter().enumerate().map(|(i, (_, (_, expense)))| {
                let bar_width = 0.3;
                Rectangle::new(
                    [(i as f32 + bar_width as f32 / 2.0) as i32, 0.0],
                    [(i as f32 + 3.0 * bar_width as f32 / 2.0) as i32, *expense],
                    RED.filled(),
                )
            })
        )?;
        
        // Add legend
        chart.configure_series_labels()
            .background_style(&WHITE)
            .border_style(&BLACK)
            .draw()?;
            
        root.draw_series(std::iter::once(Rectangle::new(
            [(area.width() - 120) as i32, 10],
            [(area.width() - 100) as i32, 20],
            GREEN.filled(),
        )))?;
        root.draw_text("Income", &("sans-serif", 12).into_font().color(&BLACK), ((area.width() - 90) as i32, 18))?;
        
        root.draw_series(std::iter::once(Rectangle::new(
            [(area.width() - 120) as i32, 30],
            [(area.width() - 100) as i32, 40],
            RED.filled(),
        )))?;
        root.draw_text("Expenses", &("sans-serif", 12).into_font().color(&BLACK), ((area.width() - 90) as i32, 38))?;
        
        Ok(())
    });

    Column::new()
        .push(
            Text::new("Income vs Expenses (6 Month Trend)")
                .size(20)
                .style(Text::Default)
        )
        .push(
            Container::new(chart.height(Length::Fixed(CHART_HEIGHT as f32)))
                .width(Length::Fixed(CHART_WIDTH as f32))
                .style(Container::Box)
        )
        .spacing(10)
        .align_items(Alignment::Center)
        .into()
}

pub fn create_budget_progress_chart(
    budgets: &[Budget],
    transactions: &[Transaction],
    categories: &[Category],
) -> Element<'static, crate::ui::Message> {
    let chart = ChartWidget::new(move |_, area| {
        let backend = plotters_iced::DrawingBackend::new(area);
        let root = backend.into_drawing_area();
        root.fill(&WHITE)?;
        
        // Filter active budgets (current month)
        let now = chrono::Local::now().naive_local().date();
        let active_budgets: Vec<&Budget> = budgets.iter()
            .filter(|b| b.start_date <= now && b.end_date >= now)
            .collect();
            
        if active_budgets.is_empty() {
            let style = TextStyle::from(("sans-serif", 20).into_font()).color(&BLACK);
            root.draw_text("No active budgets", &style, (150, 150))?;
            return Ok(());
        }
            
        let cat_map: HashMap<String, &Category> = categories
            .iter()
            .map(|c| (c.id.clone(), c))
            .collect();
            
        // Calculate spent amount for each budget
        let budget_spent: HashMap<&String, f64> = active_budgets.iter()
            .map(|budget| {
                let spent = transactions.iter()
                    .filter(|t| {
                        t.is_expense() && 
                        t.date >= budget.start_date && 
                        t.date <= budget.end_date &&
                        match (&t.category_id, &budget.category_id) {
                            (Some(t_cat), Some(b_cat)) => t_cat == b_cat,
                            _ => false
                        }
                    })
                    .map(|t| t.amount)
                    .sum();
                
                (&budget.id, spent)
            })
            .collect();
            
        let max_budget = active_budgets.iter()
            .map(|b| b.amount)
            .fold(0.0, f64::max);
            
        let mut chart = ChartBuilder::on(&root)
            .caption("Budget Progress", ("sans-serif", 20))
            .margin(5)
            .x_label_area_size(40)
            .y_label_area_size(60)
            .build_cartesian_2d(0..active_budgets.len(), 0.0..max_budget * 1.1)?;
            
        chart.configure_mesh()
            .x_labels(active_budgets.len())
            .x_label_formatter(&|idx| {
                if *idx < active_budgets.len() {
                    let budget = active_budgets[*idx];
                    budget.name.clone()
                } else {
                    "".to_string()
                }
            })
            .y_label_formatter(&|y| format!("${:.0}", y))
            .draw()?;
            
        // Draw budget bars
        chart.draw_series(
            active_budgets.iter().enumerate().map(|(i, budget)| {
                let bar_width = 0.4;
                Rectangle::new(
                    [(i as f32 - bar_width as f32 / 2.0) as i32, 0.0],
                    [(i as f32 + bar_width as f32 / 2.0) as i32, budget.amount],
                    BLUE.filled(),
                )
            })
        )?;
        
        // Draw spent bars
        chart.draw_series(
            active_budgets.iter().enumerate().map(|(i, budget)| {
                let bar_width = 0.4;
                let spent = *budget_spent.get(&budget.id).unwrap_or(&0.0);
                let color = if spent > budget.amount { RED } else { GREEN };
                
                Rectangle::new(
                    [(i as f32 + bar_width as f32 / 2.0) as i32, 0.0],
                    [(i as f32 + 3.0 * bar_width as f32 / 2.0) as i32, spent],
                    color.filled(),
                )
            })
        )?;
        
        // Add legend
        root.draw_series(std::iter::once(Rectangle::new(
            [(area.width() - 120) as i32, 10],
            [(area.width() - 100) as i32, 20],
            BLUE.filled(),
        )))?;
        root.draw_text("Budget", &("sans-serif", 12).into_font().color(&BLACK), ((area.width() - 90) as i32, 18))?;
        
        root.draw_series(std::iter::once(Rectangle::new(
            [(area.width() - 120) as i32, 30],
            [(area.width() - 100) as i32, 40],
            GREEN.filled(),
        )))?;
        root.draw_text("Spent", &("sans-serif", 12).into_font().color(&BLACK), ((area.width() - 90) as i32, 38))?;
        
        Ok(())
    });

    Column::new()
        .push(
            Text::new("Budget Progress")
                .size(20)
                .style(Text::Default)
        )
        .push(
            Container::new(chart.height(Length::Fixed(CHART_HEIGHT as f32)))
                .width(Length::Fixed(CHART_WIDTH as f32))
                .style(Container::Box)
        )
        .spacing(10)
        .align_items(Alignment::Center)
        .into()
}

pub fn spending_chart<'a, Message>(_transactions: &[Transaction]) -> Element<'a, Message> {
    iced::widget::text("Spending Chart").into()
}

pub fn budget_progress<'a, Message>(_budget: &Budget, _transactions: &[Transaction]) -> Element<'a, Message> {
    iced::widget::text("Budget Progress").into()
}

fn parse_color(hex: &str) -> plotters::style::RGBAColor {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return plotters::style::RGBAColor(100, 100, 100, 1.0);
    }
    
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(100);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(100);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(100);
    
    plotters::style::RGBAColor(r, g, b, 1.0)
}
