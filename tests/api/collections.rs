endpoint!(details: "/collection/10", "collection-details", client => client.collection(10).details());
endpoint!(images: "/collection/10/images", "collection-images", client => client.collection(10).images());
endpoint!(translations: "/collection/10/translations", "collection-translations", client => client.collection(10).translations());
