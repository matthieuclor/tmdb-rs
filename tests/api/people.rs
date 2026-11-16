use tmdb_rs::ChangesQuery;

endpoint!(details: "/person/31", "person-details", client => client.person(31).details());
endpoint!(changes: "/person/31/changes", "person-changes", client => client.person(31).changes(&ChangesQuery::default()));
endpoint!(combined_credits: "/person/31/combined_credits", "person-combined-credits", client => client.person(31).combined_credits());
endpoint!(movie_credits: "/person/31/movie_credits", "person-movie-credits", client => client.person(31).movie_credits());
endpoint!(tv_credits: "/person/31/tv_credits", "person-tv-credits", client => client.person(31).tv_credits());
endpoint!(external_ids: "/person/31/external_ids", "person-external-ids", client => client.person(31).external_ids());
endpoint!(images: "/person/31/images", "person-images", client => client.person(31).images());
endpoint!(tagged_images: "/person/31/tagged_images", "person-tagged-images", client => client.person(31).tagged_images(1));
endpoint!(translations: "/person/31/translations", "person-translations", client => client.person(31).translations());
endpoint!(latest: "/person/latest", "person-latest-id", client => client.people().latest());
endpoint!(popular: "/person/popular", "person-popular-list", client => client.people().popular(1));
endpoint!(changed: "/person/changes", "changes-people-list", client => client.people().changes(&ChangesQuery::default()));
