endpoint!(details: "/company/1", "company-details", client => client.company(1).details());
endpoint!(alternative_names: "/company/1/alternative_names", "company-alternative-names", client => client.company(1).alternative_names());
endpoint!(images: "/company/1/images", "company-images", client => client.company(1).images());
