create table word(
    word text not null primary key,
    en_uk_pronounce text not null,
    en_us_pronounce text not null,
    vi_pronounce text not null
);

create table word_type(
    id int generated always as identity,
    vi text not null,
    en text not null
);

create table word_type_link(
id int generated always as identity,
word text not null,
word_type int not null
);

create table word_meaning(
    id int generated always as identity,
    word_type_link_id int not null,
    vi_meaning text not null,
    en_meaning text not null
);

create table example(
    id int generated always as identity,
    word_meaning_id int not null,
    en_example text not null,
    vi_meaning text not null
);


danh từ
nội động từ
ngoại động từ
tính từ
phó từ
viết tắt
tiền tố
mạo từ
giới từ
trạng từ
đại từ
liên từ
thán từ

insert into word_type(vi, en)
values
('Danh từ', 'Pronounce'),
('Nội động từ', 'Intransitive verb'),
('Ngoại động từ', 'Transitive verb'),
('Tính từ', 'Adjective'),
('Phó từ', 'Adverb'),
('Viết tắt', 'Abbreviation'),
('Tiền tố', 'Prefix'),
('Mạo từ', 'Article'),
('Giới từ', 'Perposition'),
('Đại từ', 'Pronouns'),
('Liên từ','Conjunction'),
('Thán từ','Interjection')