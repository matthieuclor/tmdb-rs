endpoint!(details: "/configuration", "configuration-details", client => client.configuration().details());
endpoint!(countries: "/configuration/countries", "configuration-countries", client => client.configuration().countries());
endpoint!(jobs: "/configuration/jobs", "configuration-jobs", client => client.configuration().jobs());
endpoint!(languages: "/configuration/languages", "configuration-languages", client => client.configuration().languages());
endpoint!(primary_translations: "/configuration/primary_translations", "configuration-primary-translations", client => client.configuration().primary_translations());
endpoint!(timezones: "/configuration/timezones", "configuration-timezones", client => client.configuration().timezones());
endpoint!(watch_regions: "/watch/providers/regions", "watch-providers-available-regions", client => client.configuration().watch_regions());
