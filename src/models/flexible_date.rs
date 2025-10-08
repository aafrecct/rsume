use icu_datetime::fieldsets;
use icu_datetime::input::Date;
use icu_datetime::FixedCalendarDateTimeFormatter;
use icu_locale::Locale;
use rust_i18n::{locale, t};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(try_from = "String", into = "String")]
pub enum FlexibleDate {
    Now,
    Year(u16),
    Month(u16, u8),
    Day(u16, u8, u8),
}

impl FlexibleDate {
    fn as_tuple(&self) -> (bool, u16, u8, u8) {
        match self {
            FlexibleDate::Now => (true, 0, 0, 0),
            FlexibleDate::Year(y) => (false, *y, 0, 0),
            FlexibleDate::Month(y, m) => (false, *y, *m, 0),
            FlexibleDate::Day(y, m, d) => (false, *y, *m, *d),
        }
    }

    fn innner_cmp(&self, other: &Self) -> Ordering {
        use FlexibleDate as D;

        match (self, other) {
            (D::Now, D::Now) => Ordering::Equal,
            (D::Now, _) => Ordering::Greater,
            (_, D::Now) => Ordering::Less,
            (date_a, date_b) => date_a.as_tuple().cmp(&date_b.as_tuple()),
        }
    }
}

impl PartialOrd for FlexibleDate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FlexibleDate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.innner_cmp(other)
    }
}

impl TryFrom<String> for FlexibleDate {
    type Error = String;

    fn try_from(mut value: String) -> Result<Self, Self::Error> {
        value.retain(|c| !c.is_whitespace());
        if value.to_lowercase() == "now" {
            return Ok(FlexibleDate::Now);
        }

        let parts: Vec<&str> = value.split('-').collect();
        match parts.len() {
            1 => Ok(FlexibleDate::Year(
                parts[0]
                    .parse::<u16>()
                    .or(Err(format!("Invalid year {}", parts[0])))?,
            )),
            2 => Ok(FlexibleDate::Month(
                parts[0]
                    .parse::<u16>()
                    .or(Err(format!("Invalid year {}", parts[0])))?,
                parts[1]
                    .parse::<u8>()
                    .or(Err(format!("Invalid month {}", parts[1])))?,
            )),
            3 => Ok(FlexibleDate::Day(
                parts[0]
                    .parse::<u16>()
                    .or(Err(format!("Invalid year {}", parts[0])))?,
                parts[1]
                    .parse::<u8>()
                    .or(Err(format!("Invalid month {}", parts[1])))?,
                parts[2]
                    .parse::<u8>()
                    .or(Err(format!("Invalid day {}", parts[2])))?,
            )),
            _ => Err(format!("Invalid date: {}", value)),
        }
    }
}

impl From<FlexibleDate> for String {
    fn from(value: FlexibleDate) -> String {
        if value == FlexibleDate::Now {
            return t!("now").to_string();
        }

        let locale: Locale = locale().parse().unwrap();
        let (year, month, day) = match value {
            FlexibleDate::Year(year) => (year, 1, 1),
            FlexibleDate::Month(year, month) => (year, month, 1),
            FlexibleDate::Day(year, month, day) => (year, month, day),
            FlexibleDate::Now => unreachable!(),
        };
        let date = Date::try_new_gregorian(year as i32, month, day).unwrap();

        // The formatter is created within the match statement because formatters
        // with different fieldsets are different types and I have not found a way to create one
        // formatter and then use it to format whichever date.

        let date_str = match value {
            FlexibleDate::Year(_) => {
                let formatter =
                    FixedCalendarDateTimeFormatter::try_new(locale.into(), fieldsets::Y::medium())
                        .unwrap();
                formatter.format(&date).to_string()
            }
            FlexibleDate::Month(_, _) => {
                let formatter =
                    FixedCalendarDateTimeFormatter::try_new(locale.into(), fieldsets::YM::medium())
                        .unwrap();
                formatter.format(&date).to_string()
            }
            FlexibleDate::Day(_, _, _) => {
                let formatter = FixedCalendarDateTimeFormatter::try_new(
                    locale.into(),
                    fieldsets::YMD::medium(),
                )
                .unwrap();
                formatter.format(&date).to_string()
            }
            FlexibleDate::Now => unreachable!(),
        };

        date_str
    }
}

impl JsonSchema for FlexibleDate {
    fn schema_name() -> String {
        "FlexibleDate".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        use schemars::schema::*;
        let mut obj = SchemaObject {
            instance_type: Some(SingleOrVec::Single(InstanceType::String.into())),
            ..SchemaObject::default()
        };

        obj.string().pattern = Some(
            "^((([1-2][0-9]{3}-)?[0-1][0-9]-[0-3][0-9]|[1-2][0-9]{3}-)?[0-1][0-9]|[1-2][0-9]{3})|(now)$"
                .into(),
        );
        let obj = Schema::Object(obj);
        gen.definitions_mut()
            .insert("FlexibleDate".into(), obj.clone());

        let or = SchemaObject {
            reference: Some("#/definitions/FlexibleDate".into()),
            ..SchemaObject::default()
        };
        Schema::Object(or)
    }
}
