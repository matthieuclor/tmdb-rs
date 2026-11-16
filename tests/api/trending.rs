use tmdb_rs::TimeWindow;

endpoint!(all: "/trending/all/day", "trending-all", client => client.trending(TimeWindow::Day).all());
endpoint!(movies: "/trending/movie/week", "trending-movies", client => client.trending(TimeWindow::Week).movies());
endpoint!(tv: "/trending/tv/day", "trending-tv", client => client.trending(TimeWindow::Day).tv());
endpoint!(people: "/trending/person/week", "trending-people", client => client.trending(TimeWindow::Week).people());
