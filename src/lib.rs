#![doc = include_str!("../README.md")]

mod client;
mod collections;
mod common;
mod companies;
mod configuration;
mod credits;
mod discover;
mod episodes;
mod error;
mod find;
mod image;
mod keywords;
mod lists;
mod media;
mod movies;
mod networks;
mod people;
mod reference;
mod reviews;
mod search;
mod seasons;
mod serde_helpers;
mod trending;
mod tv;

pub use client::Client;
pub use collections::{Collection, CollectionApi, CollectionTranslation};
pub use common::{
    AggregateCast, AggregateCredits, AggregateCrew, AlternativeName, AlternativeNames,
    AlternativeTitle, AuthorDetails, Cast, Change, ChangeItem, Changes, Company, Country, Credits,
    Crew, DateRange, ExternalIds, Gender, Genre, Image, Images, Job, Keyword, Logos, Page,
    RegionWatchProviders, Review, Role, SpokenLanguage, Translation, Translations, UserList, Video,
    Videos, WatchProvider, WatchProviders,
};
pub use companies::{CompanyApi, CompanyDetails};
pub use configuration::{
    Configuration, ConfigurationApi, CountryInfo, Department, ImagesConfiguration, LanguageInfo,
    Timezone, WatchRegions,
};
pub use credits::{CreditApi, CreditDetails, CreditMedia};
pub use discover::{DiscoverApi, MovieDiscover, TvDiscover};
pub use episodes::{Episode, EpisodeApi, EpisodeAppend, EpisodeGroup, EpisodeGroupEntry};
pub use error::Error;
pub use find::{ExternalIdApi, ExternalSource, FindResults};
pub use image::image_url;
pub use keywords::KeywordApi;
pub use lists::{ItemStatus, List, ListApi};
pub use media::{CollectionSummary, Media, MovieSummary, PersonSummary, TvSummary};
pub use movies::{
    CountryReleaseDates, Movie, MovieAlternativeTitles, MovieApi, MovieAppend, MovieKeywords,
    MovieTranslation, MoviesApi, ReleaseDate, ReleaseDates,
};
pub use networks::{Network, NetworkApi};
pub use people::{
    PeopleApi, Person, PersonApi, PersonAppend, PersonCredit, PersonCredits, PersonTranslation,
    TaggedImage,
};
pub use reference::{
    Certification, Certifications, ChangedId, ChangesQuery, Genres, WatchProviderList,
};
pub use reviews::ReviewApi;
pub use search::{SearchApi, SearchQuery};
pub use seasons::{Season, SeasonApi, SeasonAppend};
pub use trending::{TimeWindow, TrendingApi};
pub use tv::{
    ContentRating, ContentRatings, Creator, EpisodeGroupSummary, EpisodeGroups, EpisodeSummary,
    ScreenedEpisode, ScreenedTheatrically, SeasonSummary, TvAlternativeTitles, TvApi, TvAppend,
    TvKeywords, TvSeries, TvShowsApi, TvTranslation,
};
