-- Your SQL goes here
Create Table facebook_log_users (
    _id Serial Primary Key,
    facebook_user_id Varchar Unique Not Null,
    facebook_user_name Varchar(50) Not Null,
    facebook_user_gender Varchar(10) Not Null,
    facebook_user_email Varchar(50) Not Null,
    facebook_user_created Timestamp Not Null Default Current_Timestamp,
    facebook_user_profile Varchar Not Null,
    facebook_user_role Varchar(10) Default 'User',
    facebook_user_phone Varchar(20)
)