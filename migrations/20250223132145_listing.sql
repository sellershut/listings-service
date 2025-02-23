create type status as enum ('active', 'sold', 'inactive');
create type item_condition as enum ('new', 'like new', 'used', 'refurbished', 'for parts');

create table listing (
    id varchar(21) primary key,
    ap_id varchar unique not null,
    user_ap_id varchar not null,
    local boolean not null,
    title varchar not null,
    description varchar not null,
    quantity integer not null,
    status status not null,
    currency varchar(3) not null,
    cost money not null,
    thumbnail varchar,
    is_unlimited boolean not null,
    media varchar[] not null default '{}',
    liked_by varchar[] not null default '{}', -- array of ids
    category_ap_id varchar not null,
    expires_at timestamptz,
    created_at timestamptz default current_timestamp not null, -- timestamp for creation
    updated_at timestamptz default current_timestamp not null -- timestamp for last update
);

create table listing_item (
    id varchar(21) primary key,
    listing_ap_id varchar not null,
    quantity int check (quantity >= 0) not null,
    condition item_condition not null,
    condition_description text default null, -- optional details (e.g., "screen scratched")
    created_at timestamp default current_timestamp not null,
    updated_at timestamp default current_timestamp not null,
    foreign key (listing_ap_id) references listing(ap_id) on delete cascade
);

create function check_listing_quantity() returns trigger as $$
begin
    if not exists (
        select 1 from listing_item where listing_ap_id = new.listing_ap_id and quantity > 0
    ) then
        update listing set status = 'sold' where ap_id = new.listing_ap_id;
    end if;
    return new;
end;
$$ language plpgsql;

create trigger trigger_check_listing_quantity
after update or delete on listing_item
for each row execute function check_listing_quantity();

create index idx_listing_id on listing (id);
create index idx_listing_ap_id on listing (ap_id);
create index idx_listing_title on listing (title);
create index idx_listing_category_ap_id on listing (category_ap_id);

create or replace function update_updated_at()
returns trigger as $$
begin
    new.updated_at = current_timestamp;
    return new;
end;
$$ language plpgsql;

create trigger set_updated_at
before update on listing
for each row
execute function update_updated_at();
