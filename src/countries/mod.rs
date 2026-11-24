use crate::{Date, Error, Holiday, HolidayType, calendar::*};
use chrono::{Datelike, Duration};
use indexmap::IndexMap;
mod albania;
mod andorra;
mod angola;
mod argentine;
mod australia;
mod austria;
mod azerbaijan;
mod bahrain;
mod bangladesh;
mod belarus;
mod belgium;
mod benin;
mod bolivia;
mod bosnia_and_herzegovina;
mod brazil;
mod bulgaria;
mod canada;
mod chile;
mod colombia;
mod croatia;
mod czechia;
mod denmark;
mod dominican_republic;
mod ecuador;
mod egypt;
mod el_salvador;
mod england;
mod estonia;
mod ethiopia;
mod finland;
mod france;
mod georgia;
mod germany;
mod ghana;
mod greece;
mod guatemala;
mod haiti;
mod honduras;
mod hong_kong;
mod hungary;
mod iceland;
mod india;
mod indonesia;
mod iran;
mod ireland;
mod italy;
mod jamaica;
mod japan;
mod kenya;
mod korea;
mod kosovo;
mod latvia;
mod liechtenstein;
mod lithuania;
mod luxembourg;
mod malawi;
mod malaysia;
mod maldives;
mod mexico;
mod moldova;
mod montenegro;
mod morocco;
mod myanmar;
mod nepal;
mod netherlands;
mod new_zealand;
mod nicaragua;
mod nigeria;
mod north_macedonia;
mod northern_ireland;
mod norway;
mod pakistan;
mod panama;
mod paraguay;
mod peru;
mod philippines;
mod poland;
mod portugal;
mod romania;
mod saudi_arabia;
mod scotland;
mod serbia;
mod singapore;
mod slovakia;
mod slovenia;
mod south_africa;
mod spain;
mod sri_lanka;
mod sweden;
mod switzerland;
mod syria;
mod taiwan;
mod tanzania;
mod tunisia;
mod turkey;
mod turkmenistan;
mod uganda;
mod ukraine;
mod united_states;
mod uzbekistan;
mod venezuela;
mod vietnam;
mod wales;
mod zambia;
pub(crate) const COUNTRIES: &[&str] = &[
    "al", "ad", "ao", "ar", "au", "at", "az", "bh", "bd", "by", "be", "bj", "bo", "ba", "br", "bg",
    "ca", "cl", "co", "hr", "cz", "dk", "do", "ec", "eg", "sv", "gb-eng", "ee", "et", "fi", "fr",
    "ge", "de", "gh", "gr", "el", "gt", "ht", "hn", "hk", "hu", "is", "in", "id", "ir", "ie", "it",
    "jm", "jp", "ke", "kr", "xk", "lv", "li", "lt", "lu", "mw", "my", "mv", "mx", "md", "me", "ma",
    "mm", "np", "nl", "nz", "ni", "ng", "mk", "gb-nir", "no", "pk", "pa", "py", "pe", "ph", "pl",
    "pt", "ro", "sa", "gb-sct", "sr", "sg", "sk", "si", "za", "es", "lk", "se", "ch", "sy", "tw",
    "tz", "tn", "tr", "tm", "ug", "ua", "us", "uz", "ve", "vn", "gb-cym", "zm",
];
pub(crate) fn calculate(country: &str, region: &str, year: i32) -> Result<Vec<Holiday>, Error> {
    match country {
        "al" => {
            if !(2024..=2034).contains(&year) {
                return Err(Error::UnsupportedYear);
            }
            albania::Rules { _region: region }.all_holidays(year)
        }
        "ad" => andorra::Rules { _region: region }.all_holidays(year),
        "ao" => angola::Rules { _region: region }.all_holidays(year),
        "ar" => argentine::Rules { _region: region }.all_holidays(year),
        "au" => australia::Rules { _region: region }.all_holidays(year),
        "at" => austria::Rules { _region: region }.all_holidays(year),
        "az" => azerbaijan::Rules { _region: region }.all_holidays(year),
        "bh" => {
            if !(2020..=2037).contains(&year) {
                return Err(Error::UnsupportedYear);
            }
            bahrain::Rules { _region: region }.all_holidays(year)
        }
        "bd" => bangladesh::Rules { _region: region }.all_holidays(year),
        "by" => belarus::Rules { _region: region }.all_holidays(year),
        "be" => belgium::Rules { _region: region }.all_holidays(year),
        "bj" => {
            if !(1970..=2037).contains(&year) {
                return Err(Error::UnsupportedYear);
            }
            benin::Rules { _region: region }.all_holidays(year)
        }
        "bo" => bolivia::Rules { _region: region }.all_holidays(year),
        "ba" => bosnia_and_herzegovina::Rules { _region: region }.all_holidays(year),
        "br" => brazil::Rules { _region: region }.all_holidays(year),
        "bg" => bulgaria::Rules { _region: region }.all_holidays(year),
        "ca" => canada::Rules { _region: region }.all_holidays(year),
        "cl" => chile::Rules { _region: region }.all_holidays(year),
        "co" => colombia::Rules { _region: region }.all_holidays(year),
        "hr" => croatia::Rules { _region: region }.all_holidays(year),
        "cz" => czechia::Rules { _region: region }.all_holidays(year),
        "dk" => denmark::Rules { _region: region }.all_holidays(year),
        "do" => dominican_republic::Rules { _region: region }.all_holidays(year),
        "ec" => ecuador::Rules { _region: region }.all_holidays(year),
        "eg" => {
            if !(2005..=2037).contains(&year) {
                return Err(Error::UnsupportedYear);
            }
            egypt::Rules { _region: region }.all_holidays(year)
        }
        "sv" => el_salvador::Rules { _region: region }.all_holidays(year),
        "gb-eng" => england::Rules { _region: region }.all_holidays(year),
        "ee" => estonia::Rules { _region: region }.all_holidays(year),
        "et" => {
            if !(1970..=2037).contains(&year) {
                return Err(Error::UnsupportedYear);
            }
            ethiopia::Rules { _region: region }.all_holidays(year)
        }
        "fi" => finland::Rules { _region: region }.all_holidays(year),
        "fr" => france::Rules { _region: region }.all_holidays(year),
        "ge" => georgia::Rules { _region: region }.all_holidays(year),
        "de" => germany::Rules { _region: region }.all_holidays(year),
        "gh" => ghana::Rules { _region: region }.all_holidays(year),
        "gr" => greece::Rules { _region: region }.all_holidays(year),
        "el" => greece::Rules { _region: region }.all_holidays(year),
        "gt" => guatemala::Rules { _region: region }.all_holidays(year),
        "ht" => haiti::Rules { _region: region }.all_holidays(year),
        "hn" => honduras::Rules { _region: region }.all_holidays(year),
        "hk" => hong_kong::Rules { _region: region }.all_holidays(year),
        "hu" => hungary::Rules { _region: region }.all_holidays(year),
        "is" => iceland::Rules { _region: region }.all_holidays(year),
        "in" => {
            if !(1970..=2037).contains(&year) {
                return Err(Error::UnsupportedYear);
            }
            india::Rules { _region: region }.all_holidays(year)
        }
        "id" => indonesia::Rules { _region: region }.all_holidays(year),
        "ir" => iran::Rules { _region: region }.all_holidays(year),
        "ie" => ireland::Rules { _region: region }.all_holidays(year),
        "it" => italy::Rules { _region: region }.all_holidays(year),
        "jm" => jamaica::Rules { _region: region }.all_holidays(year),
        "jp" => japan::Rules { _region: region }.all_holidays(year),
        "ke" => kenya::Rules { _region: region }.all_holidays(year),
        "kr" => korea::Rules { _region: region }.all_holidays(year),
        "xk" => kosovo::Rules { _region: region }.all_holidays(year),
        "lv" => latvia::Rules { _region: region }.all_holidays(year),
        "li" => liechtenstein::Rules { _region: region }.all_holidays(year),
        "lt" => lithuania::Rules { _region: region }.all_holidays(year),
        "lu" => luxembourg::Rules { _region: region }.all_holidays(year),
        "mw" => malawi::Rules { _region: region }.all_holidays(year),
        "my" => malaysia::Rules { _region: region }.all_holidays(year),
        "mv" => {
            if !(2020..=2037).contains(&year) {
                return Err(Error::UnsupportedYear);
            }
            maldives::Rules { _region: region }.all_holidays(year)
        }
        "mx" => mexico::Rules { _region: region }.all_holidays(year),
        "md" => moldova::Rules { _region: region }.all_holidays(year),
        "me" => montenegro::Rules { _region: region }.all_holidays(year),
        "ma" => morocco::Rules { _region: region }.all_holidays(year),
        "mm" => myanmar::Rules { _region: region }.all_holidays(year),
        "np" => nepal::Rules { _region: region }.all_holidays(year),
        "nl" => netherlands::Rules { _region: region }.all_holidays(year),
        "nz" => new_zealand::Rules { _region: region }.all_holidays(year),
        "ni" => nicaragua::Rules { _region: region }.all_holidays(year),
        "ng" => nigeria::Rules { _region: region }.all_holidays(year),
        "mk" => north_macedonia::Rules { _region: region }.all_holidays(year),
        "gb-nir" => northern_ireland::Rules { _region: region }.all_holidays(year),
        "no" => norway::Rules { _region: region }.all_holidays(year),
        "pk" => pakistan::Rules { _region: region }.all_holidays(year),
        "pa" => panama::Rules { _region: region }.all_holidays(year),
        "py" => paraguay::Rules { _region: region }.all_holidays(year),
        "pe" => peru::Rules { _region: region }.all_holidays(year),
        "ph" => philippines::Rules { _region: region }.all_holidays(year),
        "pl" => poland::Rules { _region: region }.all_holidays(year),
        "pt" => portugal::Rules { _region: region }.all_holidays(year),
        "ro" => romania::Rules { _region: region }.all_holidays(year),
        "sa" => {
            if !(2020..=2037).contains(&year) {
                return Err(Error::UnsupportedYear);
            }
            saudi_arabia::Rules { _region: region }.all_holidays(year)
        }
        "gb-sct" => scotland::Rules { _region: region }.all_holidays(year),
        "sr" => serbia::Rules { _region: region }.all_holidays(year),
        "sg" => {
            if !(2020..=2027).contains(&year) {
                return Err(Error::UnsupportedYear);
            }
            singapore::Rules { _region: region }.all_holidays(year)
        }
        "sk" => slovakia::Rules { _region: region }.all_holidays(year),
        "si" => slovenia::Rules { _region: region }.all_holidays(year),
        "za" => south_africa::Rules { _region: region }.all_holidays(year),
        "es" => spain::Rules { _region: region }.all_holidays(year),
        "lk" => sri_lanka::Rules { _region: region }.all_holidays(year),
        "se" => sweden::Rules { _region: region }.all_holidays(year),
        "ch" => switzerland::Rules { _region: region }.all_holidays(year),
        "sy" => {
            if !(2005..=2037).contains(&year) {
                return Err(Error::UnsupportedYear);
            }
            syria::Rules { _region: region }.all_holidays(year)
        }
        "tw" => taiwan::Rules { _region: region }.all_holidays(year),
        "tz" => tanzania::Rules { _region: region }.all_holidays(year),
        "tn" => {
            if !(2005..=2037).contains(&year) {
                return Err(Error::UnsupportedYear);
            }
            tunisia::Rules { _region: region }.all_holidays(year)
        }
        "tr" => {
            if !(1970..=2037).contains(&year) {
                return Err(Error::UnsupportedYear);
            }
            turkey::Rules { _region: region }.all_holidays(year)
        }
        "tm" => turkmenistan::Rules { _region: region }.all_holidays(year),
        "ug" => uganda::Rules { _region: region }.all_holidays(year),
        "ua" => ukraine::Rules { _region: region }.all_holidays(year),
        "us" => united_states::Rules { _region: region }.all_holidays(year),
        "uz" => uzbekistan::Rules { _region: region }.all_holidays(year),
        "ve" => venezuela::Rules { _region: region }.all_holidays(year),
        "vn" => vietnam::Rules { _region: region }.all_holidays(year),
        "gb-cym" => wales::Rules { _region: region }.all_holidays(year),
        "zm" => zambia::Rules { _region: region }.all_holidays(year),
        _ => Err(Error::InvalidCountry(country.into())),
    }
}
pub(crate) fn regions(country: &str) -> Vec<String> {
    match country {
        "au" => vec![
            "act".to_owned(),
            "nsw".to_owned(),
            "nt".to_owned(),
            "qld".to_owned(),
            "sa".to_owned(),
            "tas".to_owned(),
            "vic".to_owned(),
            "wa".to_owned(),
        ],
        "ba" => vec!["ba-rs".to_owned(), "ba-fbih".to_owned(), "ba-bd".to_owned()],
        "br" => vec![
            "BR-AC".to_owned(),
            "BR-AL".to_owned(),
            "BR-AP".to_owned(),
            "BR-AM".to_owned(),
            "BR-BA".to_owned(),
            "BR-CE".to_owned(),
            "BR-DF".to_owned(),
            "BR-ES".to_owned(),
            "BR-GO".to_owned(),
            "BR-MA".to_owned(),
            "BR-MG".to_owned(),
            "BR-MS".to_owned(),
            "BR-MT".to_owned(),
            "BR-PA".to_owned(),
            "BR-PB".to_owned(),
            "BR-PE".to_owned(),
            "BR-PI".to_owned(),
            "BR-PR".to_owned(),
            "BR-RJ".to_owned(),
            "BR-RN".to_owned(),
            "BR-RO".to_owned(),
            "BR-RR".to_owned(),
            "BR-RS".to_owned(),
            "BR-SC".to_owned(),
            "BR-SE".to_owned(),
            "BR-SP".to_owned(),
            "BR-TO".to_owned(),
        ],
        "fr" => vec![
            "FR-57".to_owned(),
            "FR-67".to_owned(),
            "FR-68".to_owned(),
            "FR-971".to_owned(),
            "FR-MF".to_owned(),
            "FR-972".to_owned(),
            "FR-973".to_owned(),
            "FR-974".to_owned(),
            "FR-976".to_owned(),
            "FR-BL".to_owned(),
        ],
        "de" => vec![
            "DE-BW".to_owned(),
            "DE-BY".to_owned(),
            "DE-BE".to_owned(),
            "DE-BB".to_owned(),
            "DE-HB".to_owned(),
            "DE-HH".to_owned(),
            "DE-HE".to_owned(),
            "DE-MV".to_owned(),
            "DE-NI".to_owned(),
            "DE-NW".to_owned(),
            "DE-RP".to_owned(),
            "DE-SL".to_owned(),
            "DE-SN".to_owned(),
            "DE-ST".to_owned(),
            "DE-SH".to_owned(),
            "DE-TH".to_owned(),
        ],
        "my" => vec![
            "jhr".to_owned(),
            "kdh".to_owned(),
            "ktn".to_owned(),
            "kul".to_owned(),
            "lbn".to_owned(),
            "mlk".to_owned(),
            "nsn".to_owned(),
            "phg".to_owned(),
            "png".to_owned(),
            "prk".to_owned(),
            "pls".to_owned(),
            "pjy".to_owned(),
            "sbh".to_owned(),
            "swk".to_owned(),
            "sgr".to_owned(),
            "trg".to_owned(),
        ],
        "pt" => vec![
            "PT-0101".to_owned(),
            "PT-0102".to_owned(),
            "PT-0103".to_owned(),
            "PT-0104".to_owned(),
            "PT-0105".to_owned(),
            "PT-0106".to_owned(),
            "PT-0107".to_owned(),
            "PT-0108".to_owned(),
            "PT-0109".to_owned(),
            "PT-0110".to_owned(),
            "PT-0111".to_owned(),
            "PT-0112".to_owned(),
            "PT-0113".to_owned(),
            "PT-0114".to_owned(),
            "PT-0115".to_owned(),
            "PT-0116".to_owned(),
            "PT-0117".to_owned(),
            "PT-0118".to_owned(),
            "PT-0119".to_owned(),
            "PT-0201".to_owned(),
            "PT-0202".to_owned(),
            "PT-0203".to_owned(),
            "PT-0204".to_owned(),
            "PT-0205".to_owned(),
            "PT-0206".to_owned(),
            "PT-0207".to_owned(),
            "PT-0208".to_owned(),
            "PT-0209".to_owned(),
            "PT-0210".to_owned(),
            "PT-0211".to_owned(),
            "PT-0212".to_owned(),
            "PT-0213".to_owned(),
            "PT-0214".to_owned(),
            "PT-0301".to_owned(),
            "PT-0302".to_owned(),
            "PT-0303".to_owned(),
            "PT-0304".to_owned(),
            "PT-0305".to_owned(),
            "PT-0306".to_owned(),
            "PT-0307".to_owned(),
            "PT-0308".to_owned(),
            "PT-0309".to_owned(),
            "PT-0310".to_owned(),
            "PT-0311".to_owned(),
            "PT-0312".to_owned(),
            "PT-0313".to_owned(),
            "PT-0314".to_owned(),
            "PT-0401".to_owned(),
            "PT-0402".to_owned(),
            "PT-0403".to_owned(),
            "PT-0404".to_owned(),
            "PT-0405".to_owned(),
            "PT-0406".to_owned(),
            "PT-0407".to_owned(),
            "PT-0408".to_owned(),
            "PT-0409".to_owned(),
            "PT-0410".to_owned(),
            "PT-0411".to_owned(),
            "PT-0412".to_owned(),
            "PT-0501".to_owned(),
            "PT-0502".to_owned(),
            "PT-0503".to_owned(),
            "PT-0504".to_owned(),
            "PT-0505".to_owned(),
            "PT-0506".to_owned(),
            "PT-0507".to_owned(),
            "PT-0508".to_owned(),
            "PT-0509".to_owned(),
            "PT-0510".to_owned(),
            "PT-0511".to_owned(),
            "PT-0601".to_owned(),
            "PT-0602".to_owned(),
            "PT-0603".to_owned(),
            "PT-0604".to_owned(),
            "PT-0605".to_owned(),
            "PT-0606".to_owned(),
            "PT-0607".to_owned(),
            "PT-0608".to_owned(),
            "PT-0609".to_owned(),
            "PT-0610".to_owned(),
            "PT-0611".to_owned(),
            "PT-0612".to_owned(),
            "PT-0613".to_owned(),
            "PT-0614".to_owned(),
            "PT-0615".to_owned(),
            "PT-0616".to_owned(),
            "PT-0617".to_owned(),
            "PT-0701".to_owned(),
            "PT-0702".to_owned(),
            "PT-0703".to_owned(),
            "PT-0704".to_owned(),
            "PT-0705".to_owned(),
            "PT-0706".to_owned(),
            "PT-0707".to_owned(),
            "PT-0708".to_owned(),
            "PT-0709".to_owned(),
            "PT-0710".to_owned(),
            "PT-0711".to_owned(),
            "PT-0712".to_owned(),
            "PT-0713".to_owned(),
            "PT-0714".to_owned(),
            "PT-0801".to_owned(),
            "PT-0802".to_owned(),
            "PT-0803".to_owned(),
            "PT-0804".to_owned(),
            "PT-0805".to_owned(),
            "PT-0806".to_owned(),
            "PT-0807".to_owned(),
            "PT-0808".to_owned(),
            "PT-0809".to_owned(),
            "PT-0810".to_owned(),
            "PT-0811".to_owned(),
            "PT-0812".to_owned(),
            "PT-0813".to_owned(),
            "PT-0814".to_owned(),
            "PT-0815".to_owned(),
            "PT-0816".to_owned(),
            "PT-0901".to_owned(),
            "PT-0902".to_owned(),
            "PT-0903".to_owned(),
            "PT-0904".to_owned(),
            "PT-0905".to_owned(),
            "PT-0906".to_owned(),
            "PT-0907".to_owned(),
            "PT-0908".to_owned(),
            "PT-0909".to_owned(),
            "PT-0910".to_owned(),
            "PT-0911".to_owned(),
            "PT-0912".to_owned(),
            "PT-0913".to_owned(),
            "PT-0914".to_owned(),
            "PT-1001".to_owned(),
            "PT-1002".to_owned(),
            "PT-1003".to_owned(),
            "PT-1004".to_owned(),
            "PT-1005".to_owned(),
            "PT-1006".to_owned(),
            "PT-1007".to_owned(),
            "PT-1008".to_owned(),
            "PT-1009".to_owned(),
            "PT-1010".to_owned(),
            "PT-1011".to_owned(),
            "PT-1012".to_owned(),
            "PT-1013".to_owned(),
            "PT-1014".to_owned(),
            "PT-1015".to_owned(),
            "PT-1016".to_owned(),
            "PT-1101".to_owned(),
            "PT-1102".to_owned(),
            "PT-1103".to_owned(),
            "PT-1104".to_owned(),
            "PT-1105".to_owned(),
            "PT-1106".to_owned(),
            "PT-1107".to_owned(),
            "PT-1108".to_owned(),
            "PT-1109".to_owned(),
            "PT-1110".to_owned(),
            "PT-1111".to_owned(),
            "PT-1112".to_owned(),
            "PT-1113".to_owned(),
            "PT-1114".to_owned(),
            "PT-1115".to_owned(),
            "PT-1116".to_owned(),
            "PT-1201".to_owned(),
            "PT-1202".to_owned(),
            "PT-1203".to_owned(),
            "PT-1204".to_owned(),
            "PT-1205".to_owned(),
            "PT-1206".to_owned(),
            "PT-1207".to_owned(),
            "PT-1208".to_owned(),
            "PT-1209".to_owned(),
            "PT-1210".to_owned(),
            "PT-1211".to_owned(),
            "PT-1212".to_owned(),
            "PT-1213".to_owned(),
            "PT-1214".to_owned(),
            "PT-1215".to_owned(),
            "PT-1301".to_owned(),
            "PT-1302".to_owned(),
            "PT-1303".to_owned(),
            "PT-1304".to_owned(),
            "PT-1305".to_owned(),
            "PT-1306".to_owned(),
            "PT-1307".to_owned(),
            "PT-1308".to_owned(),
            "PT-1309".to_owned(),
            "PT-1310".to_owned(),
            "PT-1311".to_owned(),
            "PT-1312".to_owned(),
            "PT-1313".to_owned(),
            "PT-1314".to_owned(),
            "PT-1315".to_owned(),
            "PT-1316".to_owned(),
            "PT-1317".to_owned(),
            "PT-1318".to_owned(),
            "PT-1401".to_owned(),
            "PT-1402".to_owned(),
            "PT-1403".to_owned(),
            "PT-1404".to_owned(),
            "PT-1405".to_owned(),
            "PT-1406".to_owned(),
            "PT-1407".to_owned(),
            "PT-1408".to_owned(),
            "PT-1409".to_owned(),
            "PT-1410".to_owned(),
            "PT-1411".to_owned(),
            "PT-1412".to_owned(),
            "PT-1413".to_owned(),
            "PT-1414".to_owned(),
            "PT-1415".to_owned(),
            "PT-1416".to_owned(),
            "PT-1417".to_owned(),
            "PT-1418".to_owned(),
            "PT-1419".to_owned(),
            "PT-1420".to_owned(),
            "PT-1421".to_owned(),
            "PT-1501".to_owned(),
            "PT-1502".to_owned(),
            "PT-1503".to_owned(),
            "PT-1504".to_owned(),
            "PT-1505".to_owned(),
            "PT-1506".to_owned(),
            "PT-1507".to_owned(),
            "PT-1508".to_owned(),
            "PT-1509".to_owned(),
            "PT-1510".to_owned(),
            "PT-1511".to_owned(),
            "PT-1512".to_owned(),
            "PT-1513".to_owned(),
            "PT-1601".to_owned(),
            "PT-1602".to_owned(),
            "PT-1603".to_owned(),
            "PT-1604".to_owned(),
            "PT-1605".to_owned(),
            "PT-1606".to_owned(),
            "PT-1607".to_owned(),
            "PT-1608".to_owned(),
            "PT-1609".to_owned(),
            "PT-1610".to_owned(),
            "PT-1701".to_owned(),
            "PT-1702".to_owned(),
            "PT-1703".to_owned(),
            "PT-1704".to_owned(),
            "PT-1705".to_owned(),
            "PT-1706".to_owned(),
            "PT-1707".to_owned(),
            "PT-1708".to_owned(),
            "PT-1709".to_owned(),
            "PT-1710".to_owned(),
            "PT-1711".to_owned(),
            "PT-1712".to_owned(),
            "PT-1713".to_owned(),
            "PT-1714".to_owned(),
            "PT-1801".to_owned(),
            "PT-1802".to_owned(),
            "PT-1803".to_owned(),
            "PT-1804".to_owned(),
            "PT-1805".to_owned(),
            "PT-1806".to_owned(),
            "PT-1807".to_owned(),
            "PT-1808".to_owned(),
            "PT-1809".to_owned(),
            "PT-1810".to_owned(),
            "PT-1811".to_owned(),
            "PT-1812".to_owned(),
            "PT-1813".to_owned(),
            "PT-1814".to_owned(),
            "PT-1815".to_owned(),
            "PT-1816".to_owned(),
            "PT-1817".to_owned(),
            "PT-1818".to_owned(),
            "PT-1819".to_owned(),
            "PT-1820".to_owned(),
            "PT-1821".to_owned(),
            "PT-1822".to_owned(),
            "PT-1823".to_owned(),
            "PT-1824".to_owned(),
            "PT-3101".to_owned(),
            "PT-3102".to_owned(),
            "PT-3103".to_owned(),
            "PT-3104".to_owned(),
            "PT-3105".to_owned(),
            "PT-3106".to_owned(),
            "PT-3107".to_owned(),
            "PT-3108".to_owned(),
            "PT-3109".to_owned(),
            "PT-3110".to_owned(),
            "PT-3201".to_owned(),
            "PT-4101".to_owned(),
            "PT-4201".to_owned(),
            "PT-4202".to_owned(),
            "PT-4203".to_owned(),
            "PT-4204".to_owned(),
            "PT-4205".to_owned(),
            "PT-4206".to_owned(),
            "PT-4301".to_owned(),
            "PT-4302".to_owned(),
            "PT-4401".to_owned(),
            "PT-4501".to_owned(),
            "PT-4502".to_owned(),
            "PT-4601".to_owned(),
            "PT-4602".to_owned(),
            "PT-4603".to_owned(),
            "PT-4701".to_owned(),
            "PT-4801".to_owned(),
            "PT-4802".to_owned(),
            "PT-4901".to_owned(),
        ],
        "es" => vec![
            "es-an".to_owned(),
            "es-ar".to_owned(),
            "es-as".to_owned(),
            "es-cb".to_owned(),
            "es-ce".to_owned(),
            "es-cl".to_owned(),
            "es-cm".to_owned(),
            "es-cn".to_owned(),
            "es-ct".to_owned(),
            "es-ex".to_owned(),
            "es-ga".to_owned(),
            "es-ib".to_owned(),
            "es-mc".to_owned(),
            "es-md".to_owned(),
            "es-ml".to_owned(),
            "es-nc".to_owned(),
            "es-pv".to_owned(),
            "es-ri".to_owned(),
            "es-vc".to_owned(),
        ],
        "ch" => vec![
            "ch-ag".to_owned(),
            "ch-ar".to_owned(),
            "ch-ai".to_owned(),
            "ch-bl".to_owned(),
            "ch-bs".to_owned(),
            "ch-be".to_owned(),
            "ch-fr".to_owned(),
            "ch-fr-c".to_owned(),
            "ch-fr-r".to_owned(),
            "ch-ge".to_owned(),
            "ch-gl".to_owned(),
            "ch-gr".to_owned(),
            "ch-ju".to_owned(),
            "ch-lu".to_owned(),
            "ch-ne".to_owned(),
            "ch-nw".to_owned(),
            "ch-ow".to_owned(),
            "ch-sh".to_owned(),
            "ch-sz".to_owned(),
            "ch-so".to_owned(),
            "ch-sg".to_owned(),
            "ch-ti".to_owned(),
            "ch-tg".to_owned(),
            "ch-ur".to_owned(),
            "ch-vd".to_owned(),
            "ch-vs".to_owned(),
            "ch-zg".to_owned(),
            "ch-zh".to_owned(),
        ],
        _ => Vec::new(),
    }
}
pub(crate) fn translate(country: &str, locale: Option<&str>, holidays: &mut [Holiday]) {
    type Translations = std::collections::BTreeMap<
        String,
        std::collections::BTreeMap<String, std::collections::BTreeMap<String, String>>,
    >;
    static TRANSLATIONS: std::sync::OnceLock<Translations> = std::sync::OnceLock::new();
    let translations = TRANSLATIONS.get_or_init(|| {
        serde_json::from_str(include_str!("../translations.json"))
            .expect("validated translation data")
    });
    let (country, default) = match country {
        "al" => ("al", Some("al")),
        "ad" => ("ad", None),
        "ao" => ("ao", None),
        "ar" => ("ar", None),
        "au" => ("au", None),
        "at" => ("at", None),
        "az" => ("az", Some("az")),
        "bh" => ("bh", None),
        "bd" => ("bd", None),
        "by" => ("by", Some("be")),
        "be" => ("be", Some("nl")),
        "bj" => ("bj", None),
        "bo" => ("bo", None),
        "ba" => ("ba", Some("hr")),
        "br" => ("br", None),
        "bg" => ("bg", None),
        "ca" => ("ca", None),
        "cl" => ("cl", None),
        "co" => ("co", None),
        "hr" => ("hr", None),
        "cz" => ("cz", None),
        "dk" => ("dk", None),
        "do" => ("do", None),
        "ec" => ("ec", Some("es")),
        "eg" => ("eg", None),
        "sv" => ("sv", None),
        "gb-eng" => ("gb-eng", None),
        "ee" => ("ee", None),
        "et" => ("et", None),
        "fi" => ("fi", Some("fi")),
        "fr" => ("fr", None),
        "ge" => ("ge", Some("ge")),
        "de" => ("de", Some("de")),
        "gh" => ("gh", None),
        "gr" => ("gr", None),
        "el" => ("gr", None),
        "gt" => ("gt", None),
        "ht" => ("ht", None),
        "hn" => ("hn", None),
        "hk" => ("hk", None),
        "hu" => ("hu", None),
        "is" => ("is", Some("is")),
        "in" => ("in", None),
        "id" => ("id", None),
        "ir" => ("ir", Some("fa")),
        "ie" => ("ie", None),
        "it" => ("it", None),
        "jm" => ("jm", None),
        "jp" => ("jp", None),
        "ke" => ("ke", None),
        "kr" => ("kr", None),
        "xk" => ("xk", None),
        "lv" => ("lv", None),
        "li" => ("li", None),
        "lt" => ("lt", None),
        "lu" => ("lu", None),
        "mw" => ("mw", None),
        "my" => ("my", None),
        "mv" => ("mv", Some("dv")),
        "mx" => ("mx", None),
        "md" => ("md", None),
        "me" => ("me", Some("sr")),
        "ma" => ("ma", None),
        "mm" => ("mm", None),
        "np" => ("np", None),
        "nl" => ("nl", Some("nl")),
        "nz" => ("nz", None),
        "ni" => ("ni", None),
        "ng" => ("ng", None),
        "mk" => ("mk", None),
        "gb-nir" => ("gb-nir", None),
        "no" => ("no", None),
        "pk" => ("pk", None),
        "pa" => ("pa", None),
        "py" => ("py", Some("es")),
        "pe" => ("pe", None),
        "ph" => ("ph", None),
        "pl" => ("pl", None),
        "pt" => ("pt", None),
        "ro" => ("ro", None),
        "sa" => ("sa", None),
        "gb-sct" => ("gb-sct", None),
        "sr" => ("sr", None),
        "sg" => ("sg", None),
        "sk" => ("sk", None),
        "si" => ("si", None),
        "za" => ("za", None),
        "es" => ("es", None),
        "lk" => ("lk", None),
        "se" => ("se", None),
        "ch" => ("ch", Some("de")),
        "sy" => ("sy", None),
        "tw" => ("tw", None),
        "tz" => ("tz", None),
        "tn" => ("tn", Some("en")),
        "tr" => ("tr", Some("tr")),
        "tm" => ("tm", None),
        "ug" => ("ug", None),
        "ua" => ("ua", None),
        "us" => ("us", None),
        "uz" => ("uz", Some("uz")),
        "ve" => ("ve", None),
        "vn" => ("vn", Some("vi")),
        "gb-cym" => ("gb-cym", None),
        "zm" => ("zm", None),
        _ => (country, None),
    };
    if let Some(names) = locale
        .or(default)
        .and_then(|l| translations.get(country)?.get(l))
    {
        for holiday in holidays {
            if let Some(name) = names.get(&holiday.name) {
                holiday.name.clone_from(name);
            }
        }
    }
}
