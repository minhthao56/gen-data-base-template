pub const OPTIONS: [&str; 11] = [
    "Text",
    "Number",
    "Date (yyyy/mm/dd)",
    "Date (dd/mm/yyyy)",
    "Date (mm/dd/yyyy)",
    "ID",
    "Time",
    "Boolean",
    "Manual",
    "Email",
    "Phone",
];

pub const FIELD_IS_TEXT: [&str; 9] = [
    "name",
    "contact",
    "remarks",
    "address",
    "note",
    "stress",
    "title",
    "description",
    "author",
];

pub const FIELD_IS_NUMBER: [&str; 7] = [
    "age", "amount", "quantity", "price", "total", "count", "weight",
];

pub const FIELD_IS_PHONE: [&str; 3] = ["phone", "mobile", "telephone"];

pub const FIELD_IS_EMAIL: [&str; 6] = ["email", "mail", "e-mail", "outlook", "gmail", "yahoo"];

pub const FIELD_IS_DATE: [&str; 6] = ["birthday", "date", "day", "dob", "created_at", "updated_at"];

pub const FIELD_IS_ID: [&str; 3] = ["id", "identity", "uuid"];

pub const FIELD_IS_TIME: [&str; 3] = ["time", "hour", "minute"];

pub const FIELD_IS_BOOLEAN: [&str; 3] = ["is", "has", "have"];

