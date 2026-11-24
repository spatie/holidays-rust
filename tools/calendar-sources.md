# Calendar rule sources

These are maintainer references for the corrected calculations and the independent public-API examples in `tests/holidays.rs`. Differential fixtures are a regression tool, not a legal calendar specification.

## Chile

- The [government's 2025 calendar](https://www.gob.cl/noticias/calendario-feriados-2025-festivos-irrenunciables-legales) places Good Friday on April 18, Holy Saturday on April 19 and the indigenous holiday on June 20. The religious holidays follow Western Easter, not fixed March dates.
- The [Labour Directorate's explanation of law 19.668](https://www.dt.gob.cl/portal/1628/w3-article-60252.html) specifies Monday shifts for June 29 and October 12 from 2000: Tuesday through Thursday move backward, Friday moves forward, Saturday and Sunday stay unchanged.
- [Law 20.299](https://www.bcn.cl/leychile/navegar?idNorma=279294) establishes October 31 from 2008, moved to the preceding Friday when Tuesday and the following Friday when Wednesday.
- [Law 20.215](https://www.bcn.cl/leychile/navegar?idNorma=264651) adds September 17 when Monday and September 20 when Friday. [Law 20.983](https://www.bcn.cl/leychile/Navegar?idNorma=1098384) adds January 2 when Monday and September 17 when Friday from 2017. These are conditional days, not annual September 20 holidays.
- [Law 21.357](https://www.bcn.cl/leychile/navegar?idNorma=1161743) establishes the southern winter-solstice day from 2021 and explicitly sets June 21 for 2021 only.
- The solstice uses Meeus, *Astronomical Algorithms*, chapter 27 (mean June solstice polynomial and table 27.C periodic corrections), and [NASA's TT-UT polynomials](https://eclipse.gsfc.nasa.gov/SEcat5/deltatpoly.html). Convert the predicted instant to continental Chile's June UTC-4 date. The supported prediction range ends at 2100. Future timezone legislation and one-off declarations still require review.

## Mexico

[The January 17, 2006 reform of Article 74](https://www.diputados.gob.mx/LeyesBiblio/ref/lft/LFT_ref24_17ene06.pdf) establishes the first Monday in February and third Mondays in March and November. Its second transitory article preserves March 21 in 2006, so the March Monday rule starts in 2007. Earlier years retain the fixed dates.

[The September 30, 2024 reform](https://www.diputados.gob.mx/sedia/biblio/prog_leg/Prog_leg_LXVI/002_DOF_30sep24.pdf) changes the six-year presidential transition holiday from December 1 to October 1. Dates are civil dates, not UTC instants converted to Mexico City time.

## Malaysia

The [2023 federal gazette](https://www.kabinet.gov.my/storage/2024/11/20220725_warta_hka2023.pdf) lists Deepavali on November 12. The [2024 government schedule](https://www.kabinet.gov.my/storage/2024/11/hka_2024.pdf) lists Deepavali on October 31 and the Agong birthday on June 3. [The birthday history](https://www.officeholidays.com/holidays/malaysia/birthday-of-spb-yang-di-pertuan-agong) records the first Saturday in June through 2016, September 9 in 2017 through 2019, June 8 in 2020 and the first Monday in June thereafter. The crate's historical birthday rule starts in 2001.

## Chinese-calendar dates

Use ICU4X's maintained traditional Chinese calendar calculation rather than the removed copy of ICU 76.1's astronomical implementation. [Hong Kong's official 2027 calendar](https://www.gov.hk/en/about/abouthk/holiday/2027.htm) places Lunar New Year on February 6 and its following holidays on February 8 and 9. February 7 is Sunday. The shared calculation also affects Indonesia, Korea, Malaysia, Vietnam and Taiwan's fallback outside its official-calendar window. Local lunar-calendar announcements remain authoritative.

## Greece and Orthodox Easter

[Penteli municipality's 2024 announcement](https://penteli.gov.gr/may-first-holiday/) confirms the exceptional May Day shift to May 7. Keep the existing date-only rule and regression coverage; incidental request clock times must not affect civil dates.

Orthodox Easter is calculated in the Julian calendar and then converted to Gregorian. The difference is `year / 100 - year / 400 - 2` days for the March/April dates involved, not a fixed 13 days. The [US Naval Observatory's Easter explanation](https://aa.usno.navy.mil/faq/easter) distinguishes the calendars and ecclesiastical rules. Boundary regressions cover 1900 (April 22) and 2100 (May 2); the latter requires 14 days. The shared helper affects Bulgaria, Belarus, Georgia, Greece (including `el`), Moldova, Montenegro, North Macedonia, Romania, Serbia, Ukraine and Kosovo.

## Bulgaria

[Labour Code Article 154(2)](https://www.mlsp.government.bg/uploads/37/politiki/trud/zakonodatelstvo/eng/kt-eng-0824.pdf), amended in State Gazette 105/2016 and effective in 2017, establishes substitute non-working days for fixed holidays falling on weekends. Do not apply the recurring rule retroactively to 2016. This does not reconstruct individually declared historical days off.

## Differential expectations

`tests/php-parity.json` remains the unmodified pinned PHP reference. `tests/calendar-corrections.json` records fingerprints only for the configurations affected by the rules above. Every other configuration must still match the reference. The regression test checks independently sourced dates and reform boundaries so accepting a new fingerprint alone cannot establish correctness.
