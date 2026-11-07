# Jiff migration

Application date/time handling uses native Jiff types. Direct Chrono and
Chrono-TZ dependencies are removed. GTFS dependencies may still use Chrono
internally. This crate does not resolve named timezones: explicit timestamp
offsets or civil schedule times carry the required information.
