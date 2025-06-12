# YMB Location Reference

This document outlines YMB’s primary event locations. Use this information to infer or validate the `location` field in structured calendar events, especially when the event title or summary includes the location name.

---

## 📍 Campbelltown

- **Official Name:** Campbelltown Halaqah
- **Primary Use:** Weekly Halaqahs, discussions, youth programs
- **Audience:** Local youth, especially brothers from the southwest area
- **Location Inference Keywords:** "Campbelltown", "CMC", "Minto"
- **Address:** 221 Eagleview Road, Minto
- **Notes:**
  - This is YMB’s core halaqah location
  - Events here are typically spiritual but casual in tone
  - Often includes food and sports afterward

---

## 📍 St Marys

- **Official Name:** Masjid St Marys (used for Qiyam and events)
- **Primary Use:** Qiyam-ul-Layl, Ramadan events, admin catchups
- **Audience:** Wider Sydney community and YMB team
- **Location Inference Keywords:** "St Marys", "Qiyam", "Masjid St Marys"
- **Address:** 117-119 Forrester Rd, North St Marys NSW 2760
- **Notes:**
  - This is the usual site for overnight Qiyam events
  - Atmosphere is more spiritually intense and reflective
  - Occasionally used for team meetings or task days

---

## 🧠 Location Inference Rules

- If the event `summary` or `title` mentions “Campbelltown” or “St Marys”, set that as the `location`.
- If no location is provided and no clue exists in the title, default to `null`.
- Use the clean string: `"Campbelltown"` or `"St Marys"` in the structured output.
