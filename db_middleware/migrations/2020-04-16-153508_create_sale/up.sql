-- Your SQL goes here
CREATE TABLE sale (
    id_product INTEGER,
    id_currency INTEGER,
    amount NUMERIC(8,2),
    date_sale TIMESTAMP WITHOUT TIME ZONE,
    CONSTRAINT tbl_sale_id_currenct_fkey FOREIGN KEY (id_currency)
        REFERENCES public.currency (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION,
    CONSTRAINT tbl_sale_id_product_fkey FOREIGN KEY (id_product)
        REFERENCES public.product (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
)