--
-- Permissions
--

-- Table: live.comment

grant select on live.comment to viewer;
grant insert on live.comment to bidder;

-- Table: live.show

grant select on live.show to viewer;
grant insert on live.show to seller;
grant update on live.show to seller;
