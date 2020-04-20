table! {
    tabla_moneda (id_moneda) {
        id_moneda -> Int4,
        iso_moneda -> Nullable<Varchar>,
        descripcion_moneda -> Nullable<Varchar>,
    }
}

table! {
    tabla_producto (id_producto) {
        id_producto -> Int4,
        descripcion_producto -> Nullable<Varchar>,
    }
}

table! {
    tabla_ventas (id_venta) {
        id_venta -> Int4,
        id_producto -> Nullable<Int4>,
        id_moneda -> Nullable<Int4>,
        monto -> Nullable<Numeric>,
        fecha -> Nullable<Timestamp>,
    }
}

joinable!(tabla_ventas -> tabla_moneda (id_moneda));
joinable!(tabla_ventas -> tabla_producto (id_producto));

allow_tables_to_appear_in_same_query!(
    tabla_moneda,
    tabla_producto,
    tabla_ventas,
);
