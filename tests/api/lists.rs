endpoint!(details: "/list/1", "list-details", client => client.list("1").details(1));
endpoint!(item_status: "/list/1/item_status", "list-check-item-status", client => client.list("1").item_status(11));
