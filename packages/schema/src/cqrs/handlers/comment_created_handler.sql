-- Handler: cqrs.comment_created_handler

create function cqrs.comment_created_handler(event cqrs.comment_created) returns void
language plpgsql as $$
begin
  insert into live.comment (id, author_id, show_id, text)
  values (event.id, event.author_id, event.show_id, event.text);
end; $$;
