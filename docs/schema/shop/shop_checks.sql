--
-- Checks
--

-- Check: bid_concurrent_amount_check

alter table shop.bid add constraint bid_concurrent_amount_check
check (amount > concurrent_amount);
