use time::Date;

pub struct Message {
    pub id: String,
    pub sender_name: String,
    pub receiver_name: String,
    pub date_created: Date,
}
