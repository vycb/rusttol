SELECT t.id, t.name, t.parent, t.othername, t.description, (SELECT count(*) from tol WHERE parent=t.id) as childs FROM tol t WHERE t.name LIKE 'Helico%';

select * from tol limit 15;
select count(*) as chids from tol where parent=1;
select count(*) from tol;
CREATE INDEX idx_parent ON tol (parent);
--delete from tol;
--drop table tol;
