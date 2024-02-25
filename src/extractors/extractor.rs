use std::collections::HashMap;
use crate::enums::*;

use super::{
    artstation::Artstation,
    aryion::Aryion,
    bbc::Bbc,
    behance::Behance,
    blogger::Blogger,
    booru::*,
    chapter::*,
    cyberdrop::Cyberdrop,
    danbooru::Danbooru,
    derpibooru::Derpibooru,
    deviantart::Deviantart,
    e621::E621,
    exhentai::Exhentai,
    fanbox::Fanbox,
    flickr::Flickr,
    furaffinity::Furaffinity,
    gallery::*,
    gelbooru::*,
    gofile::Gofile,
    hentaifoundry::Hentaifoundry,
    hitomi::Hitomi,
    imagechest::Imagechest,
    imgur::Imgur,
    inkbunny::Inkbunny,
    instagram::Instagram,
    itaku::Itaku,
    kemonoparty::Kemonoparty,
    khinsider::Khinsider,
    lolisafe::*,
    luscious::Luscious,
    manga::*,
    mangadex::Mangadex,
    mangapark::Mangapark,
    mastodon::Mastodon,
    misskey::Misskey,
    moebooru::*,
    newgrounds::Newgrounds,
    nijie::Nijie,
    nitter::Nitter,
    oauth::Oauth,
    paheal::Paheal,
    patreon::Patreon,
    photobucket::Photobucket,
    pillowfort::Pillowfort,
    pinterest::Pinterest,
    pixeldrain::Pixeldrain,
    pixiv::*,
    plurk::Plurk,
    postmill::Postmill,
    reactor::Reactor,
    readcomiconline::Readcomiconline,
    reddit::Reddit,
    redgifs::Redgifs,
    sankaku::*,
    skeb::Skeb,
    smugmug::Smugmug,
    steamgriddb::Steamgriddb,
    szurubooru::Szurubooru,
    tumblr::*,
    twibooru::Twibooru,
    twitter::Twitter,
    unsplash::Unsplash,
    vsco::Vsco,
    wallhaven::Wallhaven,
    weasyl::Weasyl,
    weibo::Weibo,
    ytdl::Ytdl,
    zerochan::Zerochan,
};

/// Base properties for all extractors
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ExtractorBase {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<StringOrHashMap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: Option<Directory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_directory: Option<Path>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_directory: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_parent: Option<BoolOrString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_skip: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_restrict: Option<StringOrHashMap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_replace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_remove: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_strip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_extended: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_map: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<BoolOrString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sleep: Option<Duration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sleep_extractor: Option<Duration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sleep_request: Option<Duration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netrc: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<Cookie>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies_update: Option<BoolOrPath>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<StringOrHashMap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_address: Option<SourceAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referer: Option<BoolOrString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls12: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<HashMap<String, StringOrInteger>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    // String only
    pub keywords_default: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_extractor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_http: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_transfer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blacklist: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive: Option<Path>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_pragma: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    // only list of strings
    pub postprocessors: Option<Vec<StringOrPostprocessor>>,
    // Needs example
    // only list of strings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postprocessor_options: Option<Vec<StringOrPostprocessor>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_codes: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify: Option<BoolOrString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_range: Option<StringOrList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapter_range: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_filter: Option<StringOrList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapter_filter: Option<StringOrList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_unique: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapter_unique: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_pages: Option<BoolOrString>,
}

impl ExtractorBase {
    pub fn default() -> Self {
        let mut extension_map: HashMap<String, String> = HashMap::new();
        extension_map.insert("jpeg".to_string(), "jpg".to_string());
        extension_map.insert("jpe".to_string(), "jpg".to_string());
        extension_map.insert("jfif".to_string(), "jpg".to_string());
        extension_map.insert("jif".to_string(), "jpg".to_string());
        extension_map.insert("jfi".to_string(), "jpg".to_string());

        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert("User-Agent".to_string(), "<extractor.*.user-agent>".to_string());
        headers.insert("Accept".to_string(), "*/*".to_string());
        headers.insert("Accept-Language".to_string(), "en-US,en;q=0.5".to_string());
        headers.insert("Accept-Encoding".to_string(), "gzip, deflate".to_string());
        headers.insert("Referer".to_string(), "<extractor.*.referer>".to_string());

        return ExtractorBase {
            filename: None,
            directory: None,
            base_directory: Some(Path::String("./gallery-dl/".to_string())),
            parent_directory: Some(false),
            metadata_parent: Some(BoolOrString::Bool(false)),
            parent_skip: Some(false),
            path_restrict: Some(StringOrHashMap::String("auto".to_string())),
            path_replace: Some("_".to_string()),
            path_remove: Some("\\u0000-\\u001f\\u007f".to_string()),
            path_strip: Some("auto".to_string()),
            path_extended: Some(true),
            extension_map: Some(extension_map),
            skip: Some(BoolOrString::Bool(true)),
            sleep: Some(Duration::Float(0.0)),
            sleep_extractor: Some(Duration::Float(0.0)),
            sleep_request: Some(Duration::Float(0.0)),
            username: None,
            password: None,
            netrc: Some(false),
            cookies: None,
            cookies_update: Some(BoolOrPath::Bool(true)),
            proxy: None,
            source_address: None,
            user_agent: Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/115.0".to_string()),
            browser: Some("firefox".to_string()),
            referer: Some(BoolOrString::Bool(true)),
            headers: Some(headers),
            ciphers: None,
            tls12: Some(true),
            keywords: None,
            keywords_default: Some("None".to_string()),
            metadata_url: None,
            metadata_path: None,
            metadata_extractor: None,
            metadata_http: None,
            metadata_version: None,
            category_transfer: None,
            blacklist: None,
            whitelist: None,
            archive: None,
            archive_format: None,
            archive_prefix: Some("{category}".to_string()),
            archive_pragma: None,
            postprocessors: None,
            postprocessor_options: None,
            retries: Some(4),
            retry_codes: None,
            timeout: Some(30.0),
            verify: Some(BoolOrString::Bool(true)),
            download: Some(true),
            fallback: Some(true),
            image_range: None,
            chapter_range: None,
            image_filter: None,
            chapter_filter: None,
            image_unique: Some(false),
            chapter_unique: Some(false),
            date_format: Some("%Y-%m-%dT%H:%M:%S".to_string()),
            write_pages: Some(BoolOrString::Bool(false)),
        }
    }

    pub fn new(browser: Option<String>, tls12: Option<bool>) -> Self {
        return ExtractorBase {
            filename: None,
            directory: None,
            base_directory: None,
            parent_directory: None,
            metadata_parent: None,
            parent_skip: None,
            path_restrict: None,
            path_replace: None,
            path_remove: None,
            path_strip: None,
            path_extended: None,
            extension_map: None,
            skip: None,
            sleep: None,
            sleep_extractor:None,
            sleep_request: None,
            username: None,
            password: None,
            netrc: None,
            cookies: None,
            cookies_update: None,
            proxy: None,
            source_address: None,
            user_agent: None,
            browser,
            referer: None,
            headers: None,
            ciphers: None,
            tls12,
            keywords: None,
            keywords_default: None,
            metadata_url: None,
            metadata_path: None,
            metadata_extractor: None,
            metadata_http: None,
            metadata_version: None,
            category_transfer: None,
            blacklist: None,
            whitelist: None,
            archive: None,
            archive_format: None,
            archive_prefix: None,
            archive_pragma: None,
            postprocessors: None,
            postprocessor_options: None,
            retries: None,
            retry_codes: None,
            timeout: None,
            verify: None,
            download: None,
            fallback: None,
            image_range: None,
            chapter_range: None,
            image_filter: None,
            chapter_filter: None,
            image_unique: None,
            chapter_unique: None,
            date_format: None,
            write_pages: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Extractor {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modules: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_sources: Option<Vec<Path>>,

    #[serde(rename = "2ch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _2ch: Option<_2ch>,
    #[serde(rename = "2chan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _2chan: Option<_2chan>,
    #[serde(rename = "2chen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _2chen: Option<_2chen>,
    #[serde(rename = "35photo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _35photo: Option<_35photo>,
    #[serde(rename = "3dbooru")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _3dbooru: Option<_3dbooru>,
    #[serde(rename = "4archive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _4archive: Option<_4archive>,
    #[serde(rename = "4chan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _4chan: Option<_4chan>,
    #[serde(rename = "4chanarchives")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _4chanarchives: Option<_4chanarchives>,
    #[serde(rename = "500px")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _500px: Option<_500px>,
    #[serde(rename = "8chan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _8chan: Option<_8chan>,
    #[serde(rename = "8muses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _8muses: Option<_8muses>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adultempire: Option<Adultempire>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architizer: Option<Architizer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artstation: Option<Artstation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aryion: Option<Aryion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batoto: Option<Batoto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbc: Option<Bbc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behance: Option<Behance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blogger: Option<Blogger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bluesky: Option<Bluesky>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bunkr: Option<Bunkr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catbox: Option<Catbox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chevereto: Option<Chevereto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comicvine: Option<Comicvine>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cyberdrop: Option<Cyberdrop>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub danbooru: Option<Danbooru>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub derpibooru: Option<Derpibooru>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desktopography: Option<Desktopography>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deviantart: Option<Deviantart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directlink: Option<Directlink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynastyscans: Option<Dynastyscans>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e621: Option<E621>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub erome: Option<Erome>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exhentai: Option<Exhentai>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallenangels: Option<Fallenangels>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fanbox: Option<Fanbox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fanleaks: Option<Fanleaks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fantia: Option<Fantia>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fapachi: Option<Fapachi>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fapello: Option<Fapello>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flickr: Option<Flickr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foolfuuka: Option<Foolfuuka>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foolslide: Option<Foolslide>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub furaffinity: Option<Furaffinity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuskator: Option<Fuskator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gelbooru: Option<Gelbooru>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gelbooru_v01: Option<GelbooruV01>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gelbooru_v02: Option<GelbooruV02>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gofile: Option<Gofile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hatenablog: Option<Hatenablog>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hentai2read: Option<Hentai2read>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hentaicosplays: Option<Hentaicosplays>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hentaifoundry: Option<Hentaifoundry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hentaifox: Option<Hentaifox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hentaihand: Option<Hentaihand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hentaihere: Option<Hentaihere>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hiperdex: Option<Hiperdex>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hitomi: Option<Hitomi>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hotleak: Option<Hotleak>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idolcomplex: Option<Idolcomplex>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imagebam: Option<Imagebam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imagechest: Option<Imagechest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imagefap: Option<Imagefap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imagehosts: Option<Imagehosts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imgbb: Option<Imgbb>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imgbox: Option<Imgbox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imgth: Option<Imgth>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imgur: Option<Imgur>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inkbunny: Option<Inkbunny>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instagram: Option<Instagram>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuu: Option<Issuu>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub itaku: Option<Itaku>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub itchio: Option<Itchio>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jschan: Option<Jschan>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kabeuchi: Option<Kabeuchi>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keenspot: Option<Keenspot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kemonoparty: Option<Kemonoparty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub khinsider: Option<Khinsider>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub komikcast: Option<Komikcast>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lensdump: Option<Lensdump>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lexica: Option<Lexica>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lightroom: Option<Lightroom>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livedoor: Option<Livedoor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lolisafe: Option<Lolisafe>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub luscious: Option<Luscious>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lynxchan: Option<Lynxchan>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mangadex: Option<Mangadex>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mangafox: Option<Mangafox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mangahere: Option<Mangahere>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mangakakalot: Option<Mangakakalot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manganelo: Option<Manganelo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mangapark: Option<Mangapark>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mangaread: Option<Mangaread>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mangasee: Option<Mangasee>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mangoxo: Option<Mangoxo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mastodon: Option<Mastodon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub misskey: Option<Misskey>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moebooru: Option<Moebooru>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub myhentaigallery: Option<Myhentaigallery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub myportfolio: Option<Myportfolio>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub naver: Option<Naver>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub naverwebtoon: Option<Naverwebtoon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newgrounds: Option<Newgrounds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nhentai: Option<Nhentai>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nijie: Option<Nijie>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nitter: Option<Nitter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nozomi: Option<Nozomi>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfwalbum: Option<Nsfwalbum>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth: Option<Oauth>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paheal: Option<Paheal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patreon: Option<Patreon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub philomena: Option<Philomena>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photobucket: Option<Photobucket>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photovogue: Option<Photovogue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picarto: Option<Picarto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub piczel: Option<Piczel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pillowfort: Option<Pillowfort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinterest: Option<Pinterest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pixeldrain: Option<Pixeldrain>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pixiv: Option<Pixiv>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pixnet: Option<Pixnet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plurk: Option<Plurk>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poipiku: Option<Poipiku>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poringa: Option<Poringa>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pornhub: Option<Pornhub>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pornpics: Option<Pornpics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postmill: Option<Postmill>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pururin: Option<Pururin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactor: Option<Reactor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readcomiconline: Option<Readcomiconline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reddit: Option<Reddit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redgifs: Option<Redgifs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule34us: Option<Rule34us>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sankaku: Option<Sankaku>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sankakucomplex: Option<Sankakucomplex>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seiga: Option<Seiga>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub senmanga: Option<Senmanga>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sexcom: Option<Sexcom>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shimmie2: Option<Shimmie2>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shopify: Option<Shopify>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simplyhentai: Option<Simplyhentai>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skeb: Option<Skeb>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slickpic: Option<Slickpic>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slideshare: Option<Slideshare>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smugmug: Option<Smugmug>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soundgasm: Option<Soundgasm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speakerdeck: Option<Speakerdeck>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steamgriddb: Option<Steamgriddb>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribestar: Option<Subscribestar>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub szurubooru: Option<Szurubooru>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tapas: Option<Tapas>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcbscans: Option<Tcbscans>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegraph: Option<Telegraph>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmohentai: Option<Tmohentai>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toyhouse: Option<Toyhouse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tsumino: Option<Tsumino>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tumblr: Option<Tumblr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tumblrgallery: Option<TumblrGallery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twibooru: Option<Twibooru>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter: Option<Twitter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsplash: Option<Unsplash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploadir: Option<Uploadir>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urlgalleries: Option<Urlgalleries>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urlshortener: Option<Urlshortener>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vanillarock: Option<Vanillarock>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vichan: Option<Vichan>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vipergirls: Option<Vipergirls>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vk: Option<Vk>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsco: Option<Vsco>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallhaven: Option<Wallhaven>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallpapercave: Option<Wallpapercave>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warosu: Option<Warosu>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weasyl: Option<Weasyl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webmshare: Option<Webmshare>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webtoons: Option<Webtoons>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weibo: Option<Weibo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikiart: Option<Wikiart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikifeet: Option<Wikifeet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikimedia: Option<Wikimedia>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xhamster: Option<Xhamster>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xvideos: Option<Xvideos>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ytdl: Option<Ytdl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zerochan: Option<Zerochan>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zzup: Option<Zzup>,
}

impl Extractor {
    pub fn default() -> Self {
        return Extractor {
            base: Some(ExtractorBase::default()),
            modules: None,
            module_sources: None,
            
            _2ch: Some(_2ch::new()),
            _2chan: Some(_2chan::new()),
            _2chen: Some(_2chen::new()),
            _35photo: Some(_35photo::new()),
            _3dbooru: Some(_3dbooru::new()),
            _4archive: Some(_4archive::new()),
            _4chan: Some(_4chan::new()),
            _4chanarchives: Some(_4chanarchives::new()),
            _500px: Some(_500px::new()),
            _8chan: Some(_8chan::new()),
            _8muses: Some(_8muses::new()),
            adultempire: Some(Adultempire::new()),
            architizer: Some(Architizer::new()),
            artstation: Some(Artstation::new()),
            aryion: Some(Aryion::new(None, None)),
            batoto: Some(Batoto::new()),
            bbc: Some(Bbc::new()),
            behance: Some(Behance::new()),
            blogger: Some(Blogger::new()),
            bluesky: Some(Bluesky::new(None, None)),
            bunkr: Some(Bunkr::new()),
            catbox: Some(Catbox::new()),
            chevereto: Some(Chevereto::new()),
            comicvine: Some(Comicvine::new()),
            cyberdrop: Some(Cyberdrop::new()),
            danbooru: Some(Danbooru::new(None, None)),
            derpibooru: Some(Derpibooru::new()),
            desktopography: Some(Desktopography::new()),
            deviantart: Some(Deviantart::new()),
            directlink: Some(Directlink::new()),
            dynastyscans: Some(Dynastyscans::new()),
            e621: Some(E621::new(None, None)),
            erome: Some(Erome::new()),
            exhentai: Some(Exhentai::new(None, None)),
            fallenangels: Some(Fallenangels::new()),
            fanbox: Some(Fanbox::new()),
            fanleaks: Some(Fanleaks::new()),
            fantia: Some(Fantia::new()),
            fapachi: Some(Fapachi::new()),
            fapello: Some(Fapello::new()),
            flickr: Some(Flickr::new()),
            foolfuuka: Some(Foolfuuka::new()),
            foolslide: Some(Foolslide::new()),
            furaffinity: Some(Furaffinity::new()),
            fuskator: Some(Fuskator::new()),
            gelbooru: Some(Gelbooru::new()),
            gelbooru_v01: Some(GelbooruV01::new()),
            gelbooru_v02: Some(GelbooruV02::new()),
            gofile: Some(Gofile::new()),
            hatenablog: Some(Hatenablog::new()),
            hentai2read: Some(Hentai2read::new()),
            hentaicosplays: Some(Hentaicosplays::new()),
            hentaifoundry: Some(Hentaifoundry::new()),
            hentaifox: Some(Hentaifox::new()),
            hentaihand: Some(Hentaihand::new()),
            hentaihere: Some(Hentaihere::new()),
            hiperdex: Some(Hiperdex::new()),
            hitomi: Some(Hitomi::new()),
            hotleak: Some(Hotleak::new()),
            idolcomplex: Some(Idolcomplex::new(None, None)),
            imagebam: Some(Imagebam::new()),
            imagechest: Some(Imagechest::new()),
            imagefap: Some(Imagefap::new()),
            imagehosts: Some(Imagehosts::new()),
            imgbb: Some(Imgbb::new(None, None)),
            imgbox: Some(Imgbox::new()),
            imgth: Some(Imgth::new()),
            imgur: Some(Imgur::new()),
            inkbunny: Some(Inkbunny::new(None, None)),
            instagram: Some(Instagram::new()),
            issuu: Some(Issuu::new()),
            itaku: Some(Itaku::new()),
            itchio: Some(Itchio::new()),
            jschan: Some(Jschan::new()),
            kabeuchi: Some(Kabeuchi::new()),
            keenspot: Some(Keenspot::new()),
            kemonoparty: Some(Kemonoparty::new(None, None)),
            khinsider: Some(Khinsider::new()),
            komikcast: Some(Komikcast::new()),
            lensdump: Some(Lensdump::new()),
            lexica: Some(Lexica::new()),
            lightroom: Some(Lightroom::new()),
            livedoor: Some(Livedoor::new()),
            lolisafe: Some(Lolisafe::new()),
            luscious: Some(Luscious::new()),
            lynxchan: Some(Lynxchan::new()),
            mangadex: Some(Mangadex::new(None, None)),
            mangafox: Some(Mangafox::new()),
            mangahere: Some(Mangahere::new()),
            mangakakalot: Some(Mangakakalot::new()),
            manganelo: Some(Manganelo::new()),
            mangapark: Some(Mangapark::new()),
            mangaread: Some(Mangaread::new()),
            mangasee: Some(Mangasee::new()),
            mangoxo: Some(Mangoxo::new(None, None)),
            mastodon: Some(Mastodon::new()),
            misskey: Some(Misskey::new()),
            moebooru: Some(Moebooru::new()),
            myhentaigallery: Some(Myhentaigallery::new()),
            myportfolio: Some(Myportfolio::new()),
            naver: Some(Naver::new()),
            naverwebtoon: Some(Naverwebtoon::new()),
            newgrounds: Some(Newgrounds::new()),
            nhentai: Some(Nhentai::new()),
            nijie: Some(Nijie::new(None, None)),
            nitter: Some(Nitter::new()),
            nozomi: Some(Nozomi::new()),
            nsfwalbum: Some(Nsfwalbum::new()),
            oauth: Some(Oauth::new()),
            paheal: Some(Paheal::new()),
            patreon: Some(Patreon::new()),
            philomena: Some(Philomena::new()),
            photobucket: Some(Photobucket::new()),
            photovogue: Some(Photovogue::new()),
            picarto: Some(Picarto::new()),
            piczel: Some(Piczel::new()),
            pillowfort: Some(Pillowfort::new(None, None)),
            pinterest: Some(Pinterest::new()),
            pixeldrain: Some(Pixeldrain::new()),
            pixiv: Some(Pixiv::new()),
            pixnet: Some(Pixnet::new()),
            plurk: Some(Plurk::new()),
            poipiku: Some(Poipiku::new()),
            poringa: Some(Poringa::new()),
            pornhub: Some(Pornhub::new()),
            pornpics: Some(Pornpics::new()),
            postmill: Some(Postmill::new()),
            pururin: Some(Pururin::new()),
            reactor: Some(Reactor::new()),
            readcomiconline: Some(Readcomiconline::new()),
            reddit: Some(Reddit::new()),
            redgifs: Some(Redgifs::new()),
            rule34us: Some(Rule34us::new()),
            sankaku: Some(Sankaku::new(None, None)),
            sankakucomplex: Some(Sankakucomplex::new()),
            seiga: Some(Seiga::new()),
            senmanga: Some(Senmanga::new()),
            sexcom: Some(Sexcom::new()),
            shimmie2: Some(Shimmie2::new()),
            shopify: Some(Shopify::new()),
            simplyhentai: Some(Simplyhentai::new()),
            skeb: Some(Skeb::new()),
            slickpic: Some(Slickpic::new()),
            slideshare: Some(Slideshare::new()),
            smugmug: Some(Smugmug::new()),
            soundgasm: Some(Soundgasm::new()),
            speakerdeck: Some(Speakerdeck::new()),
            steamgriddb: Some(Steamgriddb::new()),
            subscribestar: Some(Subscribestar::new(None, None)),
            szurubooru: Some(Szurubooru::new()),
            tapas: Some(Tapas::new(None, None)),
            tcbscans: Some(Tcbscans::new()),
            telegraph: Some(Telegraph::new()),
            tmohentai: Some(Tmohentai::new()),
            toyhouse: Some(Toyhouse::new()),
            tsumino: Some(Tsumino::new(None, None)),
            tumblr: Some(Tumblr::new()),
            tumblrgallery: Some(TumblrGallery::new()),
            twibooru: Some(Twibooru::new()),
            twitter: Some(Twitter::new(None, None)),
            unsplash: Some(Unsplash::new()),
            uploadir: Some(Uploadir::new()),
            urlgalleries: Some(Urlgalleries::new()),
            urlshortener: Some(Urlshortener::new()),
            vanillarock: Some(Vanillarock::new()),
            vichan: Some(Vichan::new()),
            vipergirls: Some(Vipergirls::new()),
            vk: Some(Vk::new()),
            vsco: Some(Vsco::new()),
            wallhaven: Some(Wallhaven::new()),
            wallpapercave: Some(Wallpapercave::new()),
            warosu: Some(Warosu::new()),
            weasyl: Some(Weasyl::new()),
            webmshare: Some(Webmshare::new()),
            webtoons: Some(Webtoons::new()),
            weibo: Some(Weibo::new()),
            wikiart: Some(Wikiart::new()),
            wikifeet: Some(Wikifeet::new()),
            wikimedia: Some(Wikimedia::new()),
            xhamster: Some(Xhamster::new()),
            xvideos: Some(Xvideos::new()),
            ytdl: Some(Ytdl::new()),
            zerochan: Some(Zerochan::new(None, None)),
            zzup: Some(Zzup::new()),
        };
    }

    pub fn new() -> Self {
        return Extractor {
            base: Some(ExtractorBase::default()),
            modules: None,
            module_sources: None,

            _2ch: None,
            _2chan: None,
            _2chen: None,
            _35photo: None,
            _3dbooru: None,
            _4archive: None,
            _4chan: None,
            _4chanarchives: None,
            _500px: None,
            _8chan: None,
            _8muses: None,
            adultempire: None,
            architizer: None,
            artstation: None,
            aryion: None,
            batoto: None,
            bbc: None,
            behance: None,
            blogger: None,
            bluesky: None,
            bunkr: None,
            catbox: None,
            chevereto: None,
            comicvine: None,
            cyberdrop: None,
            danbooru: None,
            derpibooru: None,
            desktopography: None,
            deviantart: None,
            directlink: None,
            dynastyscans: None,
            e621: None,
            erome: None,
            exhentai: None,
            fallenangels: None,
            fanbox: None,
            fanleaks: None,
            fantia: None,
            fapachi: None,
            fapello: None,
            flickr: None,
            foolfuuka: None,
            foolslide: None,
            furaffinity: None,
            fuskator: None,
            gelbooru: None,
            gelbooru_v01: None,
            gelbooru_v02: None,
            gofile: None,
            hatenablog: None,
            hentai2read: None,
            hentaicosplays: None,
            hentaifoundry: None,
            hentaifox: None,
            hentaihand: None,
            hentaihere: None,
            hiperdex: None,
            hitomi: None,
            hotleak: None,
            idolcomplex: None,
            imagebam: None,
            imagechest: None,
            imagefap: None,
            imagehosts: None,
            imgbb: None,
            imgbox: None,
            imgth: None,
            imgur: None,
            inkbunny: None,
            instagram: None,
            issuu: None,
            itaku: None,
            itchio: None,
            jschan: None,
            kabeuchi: None,
            keenspot: None,
            kemonoparty: None,
            khinsider: None,
            komikcast: None,
            lensdump: None,
            lexica: None,
            lightroom: None,
            livedoor: None,
            lolisafe: None,
            luscious: None,
            lynxchan: None,
            mangadex: None,
            mangafox: None,
            mangahere: None,
            mangakakalot: None,
            manganelo: None,
            mangapark: None,
            mangaread: None,
            mangasee: None,
            mangoxo: None,
            mastodon: None,
            misskey: None,
            moebooru: None,
            myhentaigallery: None,
            myportfolio: None,
            naver: None,
            naverwebtoon: None,
            newgrounds: None,
            nhentai: None,
            nijie: None,
            nitter: None,
            nozomi: None,
            nsfwalbum: None,
            oauth: None,
            paheal: None,
            patreon: None,
            philomena: None,
            photobucket: None,
            photovogue: None,
            picarto: None,
            piczel: None,
            pillowfort: None,
            pinterest: None,
            pixeldrain: None,
            pixiv: None,
            pixnet: None,
            plurk: None,
            poipiku: None,
            poringa: None,
            pornhub: None,
            pornpics: None,
            postmill: None,
            pururin: None,
            reactor: None,
            readcomiconline: None,
            reddit: None,
            redgifs: None,
            rule34us: None,
            sankaku: None,
            sankakucomplex: None,
            seiga: None,
            senmanga: None,
            sexcom: None,
            shimmie2: None,
            shopify: None,
            simplyhentai: None,
            skeb: None,
            slickpic: None,
            slideshare: None,
            smugmug: None,
            soundgasm: None,
            speakerdeck: None,
            steamgriddb: None,
            subscribestar: None,
            szurubooru: None,
            tapas: None,
            tcbscans: None,
            telegraph: None,
            tmohentai: None,
            toyhouse: None,
            tsumino: None,
            tumblr: None,
            tumblrgallery: None,
            twibooru: None,
            twitter: None,
            unsplash: None,
            uploadir: None,
            urlgalleries: None,
            urlshortener: None,
            vanillarock: None,
            vichan: None,
            vipergirls: None,
            vk: None,
            vsco: None,
            wallhaven: None,
            wallpapercave: None,
            warosu: None,
            weasyl: None,
            webmshare: None,
            webtoons: None,
            weibo: None,
            wikiart: None,
            wikifeet: None,
            wikimedia: None,
            xhamster: None,
            xvideos: None,
            ytdl: None,
            zerochan: None,
            zzup: None,
        }
    }
}

// Children of BaseExtractor

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename = "2ch")]
pub struct _2ch {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl _2ch {
    pub fn new() -> Self {
        return _2ch {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename = "2chan")]
pub struct _2chan {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl _2chan {
    pub fn new() -> Self {
        return _2chan {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename = "2chen")]
pub struct _2chen {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl _2chen {
    pub fn new() -> Self {
        return _2chen {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename = "35photo")]
pub struct _35photo {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl _35photo {
    pub fn new() -> Self {
        return _35photo {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename = "4archive")]
pub struct _4archive {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl _4archive {
    pub fn new() -> Self {
        return _4archive {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename = "4chan")]
pub struct _4chan {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl _4chan {
    pub fn new() -> Self {
        return _4chan {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename = "4chanarchives")]
pub struct _4chanarchives {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl _4chanarchives {
    pub fn new() -> Self {
        return _4chanarchives {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename = "500px")]
pub struct _500px {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl _500px {
    pub fn new() -> Self {
        return _500px {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename = "8chan")]
pub struct _8chan {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}


impl _8chan {
    pub fn new() -> Self {
        return _8chan {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename = "8muses")]
pub struct _8muses {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl _8muses {
    pub fn new() -> Self {
        return _8muses {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Bluesky {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Bluesky {
    pub fn new(username: Option<String>, password: Option<String>) -> Self {
        let mut base = ExtractorBase::new(None, None);
        base.username = username;
        base.password = password;

        return Bluesky {
            base: Some(base),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Chevereto {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Chevereto {
    pub fn new() -> Self {
        return Chevereto {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Desktopography {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Desktopography {
    pub fn new() -> Self {
        return Desktopography {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Directlink {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Directlink {
    pub fn new() -> Self {
        return Directlink {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Erome {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Erome {
    pub fn new() -> Self {
        return Erome {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Fanleaks {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Fanleaks {
    pub fn new() -> Self {
        return Fanleaks {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Fantia {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Fantia {
    pub fn new() -> Self {
        return Fantia {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Fapachi {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Fapachi {
    pub fn new() -> Self {
        return Fapachi {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Fapello {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Fapello {
    pub fn new() -> Self {
        return Fapello {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Foolfuuka {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Foolfuuka {
    pub fn new() -> Self {
        return Foolfuuka {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Foolslide {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Foolslide {
    pub fn new() -> Self {
        return Foolslide {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Hatenablog {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Hatenablog {
    pub fn new() -> Self {
        return Hatenablog {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Hotleak {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Hotleak {
    pub fn new() -> Self {
        return Hotleak {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Imagebam {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Imagebam {
    pub fn new() -> Self {
        return Imagebam {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Imagefap {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Imagefap {
    pub fn new() -> Self {
        return Imagefap {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Imagehosts {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Imagehosts {
    pub fn new() -> Self {
        return Imagehosts {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Imgbb {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Imgbb {
    pub fn new(username: Option<String>, password: Option<String>) -> Self {
        let mut base = ExtractorBase::new(None, None);
        base.username = username;
        base.password = password;

        return Imgbb {
            base: Some(base),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Imgbox {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Imgbox {
    pub fn new() -> Self {
        return Imgbox {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Itchio {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Itchio {
    pub fn new() -> Self {
        return Itchio {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Jschan {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Jschan {
    pub fn new() -> Self {
        return Jschan {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Kabeuchi {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Kabeuchi {
    pub fn new() -> Self {
        return Kabeuchi {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Keenspot {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Keenspot {
    pub fn new() -> Self {
        return Keenspot {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Lexica {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Lexica {
    pub fn new() -> Self {
        return Lexica {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Lightroom {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Lightroom {
    pub fn new() -> Self {
        return Lightroom {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Livedoor {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Livedoor {
    pub fn new() -> Self {
        return Livedoor {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Lynxchan {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Lynxchan {
    pub fn new() -> Self {
        return Lynxchan {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Mangoxo {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Mangoxo {
    pub fn new(username: Option<String>, password: Option<String>) -> Self {
        let mut base = ExtractorBase::new(None, None);
        base.username = username;
        base.password = password;

        return Mangoxo {
            base: Some(base),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Myportfolio {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Myportfolio {
    pub fn new() -> Self {
        return Myportfolio {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Nozomi {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Nozomi {
    pub fn new() -> Self {
        return Nozomi {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Photovogue {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Photovogue {
    pub fn new() -> Self {
        return Photovogue {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Picarto {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Picarto {
    pub fn new() -> Self {
        return Picarto {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Piczel {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Piczel {
    pub fn new() -> Self {
        return Piczel {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Pixnet {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Pixnet {
    pub fn new() -> Self {
        return Pixnet {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Poipiku {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Poipiku {
    pub fn new() -> Self {
        return Poipiku {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Poringa {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Poringa {
    pub fn new() -> Self {
        return Poringa {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Pornhub {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Pornhub {
    pub fn new() -> Self {
        return Pornhub {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Seiga {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Seiga {
    pub fn new() -> Self {
        return Seiga {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Sexcom {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Sexcom {
    pub fn new() -> Self {
        return Sexcom {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Shimmie2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Shimmie2 {
    pub fn new() -> Self {
        return Shimmie2 {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Shopify {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Shopify {
    pub fn new() -> Self {
        return Shopify {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Slickpic {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Slickpic {
    pub fn new() -> Self {
        return Slickpic {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Soundgasm {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Soundgasm {
    pub fn new() -> Self {
        return Soundgasm {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Speakerdeck {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Speakerdeck {
    pub fn new() -> Self {
        return Speakerdeck {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Subscribestar {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Subscribestar {
    pub fn new(username: Option<String>, password: Option<String>) -> Self {
        let mut base = ExtractorBase::new(None, None);
        base.username = username;
        base.password = password;

        return Subscribestar {
            base: Some(base),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Tapas {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Tapas {
    pub fn new(username: Option<String>, password: Option<String>) -> Self {
        let mut base = ExtractorBase::new(None, None);
        base.username = username;
        base.password = password;

        return Tapas {
            base: Some(base),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Toyhouse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Toyhouse {
    pub fn new() -> Self {
        return Toyhouse {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Uploadir {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Uploadir {
    pub fn new() -> Self {
        return Uploadir {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Urlshortener {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Urlshortener {
    pub fn new() -> Self {
        return Urlshortener {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Vanillarock {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Vanillarock {
    pub fn new() -> Self {
        return Vanillarock {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Vichan {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Vichan {
    pub fn new() -> Self {
        return Vichan {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Vipergirls {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Vipergirls {
    pub fn new() -> Self {
        return Vipergirls {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Vk {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Vk {
    pub fn new() -> Self {
        return Vk {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Wallpapercave {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Wallpapercave {
    pub fn new() -> Self {
        return Wallpapercave {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Warosu {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Warosu {
    pub fn new() -> Self {
        return Warosu {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Webmshare {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Webmshare {
    pub fn new() -> Self {
        return Webmshare {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Wikiart {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Wikiart {
    pub fn new() -> Self {
        return Wikiart {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Wikimedia {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Wikimedia {
    pub fn new() -> Self {
        return Wikimedia {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Xhamster {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

impl Xhamster {
    pub fn new() -> Self {
        return Xhamster {
            base: Some(ExtractorBase::new(None, None)),
        }
    }
}
