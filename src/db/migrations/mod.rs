macro_rules! migrations {
    ($(($m:ident, $ver: expr, $name:ident)),*) => (

        // FIMXE Move this to `deuterium-orm` after it will work there.
        #[allow(unused_mut)]
        pub fn migrations<'a>() -> ::deuterium_orm::migration::Migrations {
            let mut migrations = Vec::new();

            $(
                mod $m;

                let name = stringify!($name);
                let migration = ::deuterium_orm::migration::Migration::new(
                    $ver,
                    name,
                    Box::new($m::$name) as Box<::deuterium_orm::migration::RawMigration<::postgres::Connection>>
                );

                migrations.push(Box::new(migration));
            )*

            migrations
        }

    )
}

load_migrations! {
    "src/db/migrations"
}