//Categories
db.getCollection('categories').insertMany(
  [
    {
      "_id": ObjectId("61f49736177d9513a4c7a7c1"),
      "name": "Electronics"
    },
    {
      "_id": ObjectId("61f49736177d9513a4c7a7c2"),
      "name": "Clothing"
    }
  ]
);

/**
 * Products
 */
db.getCollection('products').insertMany(
  [
    {
      "_id": ObjectId("61f49736177d9513a4c7a7b1"),
      "name": "Laptop",
      "category_id": ObjectId("61f49736177d9513a4c7a7c1"),
      "price": 999.99
    },
    {
      "_id": ObjectId("61f49736177d9513a4c7a7b2"),
      "name": "Smartphone",
      "category_id": ObjectId("61f49736177d9513a4c7a7c1"),
      "price": 599.99
    },
    {
      "_id": ObjectId("61f49736177d9513a4c7a7b3"),
      "name": "T-Shirt",
      "category_id": ObjectId("61f49736177d9513a4c7a7c2"),
      "price": 19.99
    },
    {
      "_id": ObjectId("61f49736177d9513a4c7a7b4"),
      "name": "Jeans",
      "category_id": ObjectId("61f49736177d9513a4c7a7c2"),
      "price": 39.99
    }
  ]
)
/**
 * Customers
 * */
db.getCollection('customers').insertMany(
  [
    {
      "_id": ObjectId("61f49636177d9513a4c7a7a1"),
      "name": "Alice Johnson"
    },
    {
      "_id": ObjectId("61f49636177d9513a4c7a7a2"),
      "name": "Bob Smith"
    },
    {
      "_id": ObjectId("61f49636177d9513a4c7a7a3"),
      "name": "Charlie Brown"
    }
  ]
)

/**
 * Orders
 */
db.getCollection('orders').insertMany(
  [
    {
      "_id": ObjectId("61f4981d177d9513a4c7a7d1"),
      "customer_id": ObjectId("61f49636177d9513a4c7a7a1"),
      "order_date": ISODate("2024-01-15T00:00:00Z")
    },
    {
      "_id": ObjectId("61f4981d177d9513a4c7a7d2"),
      "customer_id": ObjectId("61f49636177d9513a4c7a7a1"),
      "order_date": ISODate("2024-01-20T00:00:00Z")
    },
    {
      "_id": ObjectId("61f4981d177d9513a4c7a7d3"),
      "customer_id": ObjectId("61f49636177d9513a4c7a7a1"),
      "order_date": ISODate("2024-01-25T00:00:00Z")
    },
    {
      "_id": ObjectId("61f4981d177d9513a4c7a7d4"),
      "customer_id": ObjectId("61f49636177d9513a4c7a7a2"),
      "order_date": ISODate("2024-02-01T00:00:00Z")
    },
    {
      "_id": ObjectId("61f4981d177d9513a4c7a7d5"),
      "customer_id": ObjectId("61f49636177d9513a4c7a7a2"),
      "order_date": ISODate("2024-02-05T00:00:00Z")
    },
    {
      "_id": ObjectId("61f4981d177d9513a4c7a7d6"),
      "customer_id": ObjectId("61f49636177d9513a4c7a7a2"),
      "order_date": ISODate("2024-02-10T00:00:00Z")
    },
    {
      "_id": ObjectId("61f4981d177d9513a4c7a7d7"),
      "customer_id": ObjectId("61f49636177d9513a4c7a7a3"),
      "order_date": ISODate("2024-01-18T00:00:00Z")
    },
    {
      "_id": ObjectId("61f4981d177d9513a4c7a7d8"),
      "customer_id": ObjectId("61f49636177d9513a4c7a7a3"),
      "order_date": ISODate("2024-01-23T00:00:00Z")
    },
    {
      "_id": ObjectId("61f4981d177d9513a4c7a7d9"),
      "customer_id": ObjectId("61f49636177d9513a4c7a7a3"),
      "order_date": ISODate("2024-01-28T00:00:00Z")
    }
  ]
)

/**Order details */
db.getCollection('order_details').insertMany(
  [
    {
      "_id": ObjectId("61f4981d177d9513a4c7a7e1"),
      "order_id": ObjectId("61f4981d177d9513a4c7a7d1"),
      "product_id": ObjectId("61f49736177d9513a4c7a7b1"), // Laptop
      "quantity": 2
    },
    {
      "_id": ObjectId("61f4981d177d9513a4c7a7e2"),
      "order_id": ObjectId("61f4981d177d9513a4c7a7d1"),
      "product_id": ObjectId("61f49736177d9513a4c7a7b2"), // Smartphone
      "quantity": 1
    },
    {
      "_id": ObjectId("61f4981d177d9513a4c7a7e3"),
      "order_id": ObjectId("61f4981d177d9513a4c7a7d1"),
      "product_id": ObjectId("61f49736177d9513a4c7a7b3"), // T-Shirt
      "quantity": 3
    },
    {
      "_id": ObjectId("61f4981d177d9513a4c7a7e4"),
      "order_id": ObjectId("61f4981d177d9513a4c7a7d2"),
      "product_id": ObjectId("61f49736177d9513a4c7a7b2"), // Smartphone
      "quantity": 2
    },
    {
      "_id": ObjectId("61f4981d177d9513a4c7a7e5"),
      "order_id": ObjectId("61f4981d177d9513a4c7a7d2"),
      "product_id": ObjectId("61f49736177d9513a4c7a7b4"), // Jeans
      "quantity": 1
    },
    {
      "_id": ObjectId("61f4981d177d9513a4c7a7e6"),
      "order_id": ObjectId("61f4981d177d9513a4c7a7d2"),
      "product_id": ObjectId("61f49736177d9513a4c7a7b3"), // T-Shirt
      "quantity": 2
    },
    {
      "_id": ObjectId("61f4981d177d9513a4c7a7e7"),
      "order_id": ObjectId("61f4981d177d9513a4c7a7d3"),
      "product_id": ObjectId("61f49736177d9513a4c7a7b1"), // Laptop
      "quantity": 1
    },
    {
      "_id": ObjectId("61f4981d177d9513a4c7a7e8"),
      "order_id": ObjectId("61f4981d177d9513a4c7a7d3"),
      "product_id": ObjectId("61f49736177d9513a4c7a7b4"), // Jeans
      "quantity": 3
    },
    {
      "_id": ObjectId("61f4981d177d9513a4c7a7e9"),
      "order_id": ObjectId("61f4981d177d9513a4c7a7d3"),
      "product_id": ObjectId("61f49736177d9513a4c7a7b3"), // T-Shirt
      "quantity": 2
    }
  ]
)
