--
-- PostgreSQL database dump
--

-- Dumped from database version 12.4
-- Dumped by pg_dump version 12.4

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: codes; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.codes (
    id integer NOT NULL,
    code character varying(191) NOT NULL,
    is_used boolean DEFAULT false NOT NULL,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone
);


ALTER TABLE public.codes OWNER TO postgres;

--
-- Name: codes_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.codes_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.codes_id_seq OWNER TO postgres;

--
-- Name: codes_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.codes_id_seq OWNED BY public.codes.id;


--
-- Name: election_slot; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.election_slot (
    id integer NOT NULL,
    slot_id integer NOT NULL,
    election_id integer NOT NULL,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone
);


ALTER TABLE public.election_slot OWNER TO postgres;

--
-- Name: election_slot_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.election_slot_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.election_slot_id_seq OWNER TO postgres;

--
-- Name: election_slot_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.election_slot_id_seq OWNED BY public.election_slot.id;


--
-- Name: elections; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.elections (
    id integer NOT NULL,
    title character varying(191),
    status integer DEFAULT 0 NOT NULL,
    start timestamp(0) without time zone NOT NULL,
    "end" timestamp(0) without time zone,
    can_see_result boolean DEFAULT false NOT NULL,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone
);


ALTER TABLE public.elections OWNER TO postgres;

--
-- Name: elections_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.elections_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.elections_id_seq OWNER TO postgres;

--
-- Name: elections_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.elections_id_seq OWNED BY public.elections.id;


--
-- Name: ipvalidations; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.ipvalidations (
    id integer NOT NULL,
    ip character varying(191) NOT NULL,
    election_id integer NOT NULL,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone
);


ALTER TABLE public.ipvalidations OWNER TO postgres;

--
-- Name: ipvalidations_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.ipvalidations_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.ipvalidations_id_seq OWNER TO postgres;

--
-- Name: ipvalidations_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.ipvalidations_id_seq OWNED BY public.ipvalidations.id;


--
-- Name: members; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.members (
    id integer NOT NULL,
    member_id character varying(191) NOT NULL,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone
);


ALTER TABLE public.members OWNER TO postgres;

--
-- Name: members_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.members_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.members_id_seq OWNER TO postgres;

--
-- Name: members_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.members_id_seq OWNED BY public.members.id;


--
-- Name: migrations; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.migrations (
    id integer NOT NULL,
    migration character varying(191) NOT NULL,
    batch integer NOT NULL
);


ALTER TABLE public.migrations OWNER TO postgres;

--
-- Name: migrations_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.migrations_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.migrations_id_seq OWNER TO postgres;

--
-- Name: migrations_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.migrations_id_seq OWNED BY public.migrations.id;


--
-- Name: nominee_slot; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.nominee_slot (
    id integer NOT NULL,
    slot_id integer NOT NULL,
    nominee_id integer NOT NULL,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone
);


ALTER TABLE public.nominee_slot OWNER TO postgres;

--
-- Name: nominee_slot_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.nominee_slot_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.nominee_slot_id_seq OWNER TO postgres;

--
-- Name: nominee_slot_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.nominee_slot_id_seq OWNED BY public.nominee_slot.id;


--
-- Name: nominees; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.nominees (
    id integer NOT NULL,
    first_name character varying(60) NOT NULL,
    last_name character varying(60) NOT NULL,
    email character varying(191),
    image character varying(191),
    description text,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone
);


ALTER TABLE public.nominees OWNER TO postgres;

--
-- Name: nominees_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.nominees_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.nominees_id_seq OWNER TO postgres;

--
-- Name: nominees_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.nominees_id_seq OWNED BY public.nominees.id;


--
-- Name: password_resets; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.password_resets (
    email character varying(191) NOT NULL,
    token character varying(191) NOT NULL,
    created_at timestamp(0) without time zone
);


ALTER TABLE public.password_resets OWNER TO postgres;

--
-- Name: positions; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.positions (
    id integer NOT NULL,
    name character varying(60) NOT NULL,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone
);


ALTER TABLE public.positions OWNER TO postgres;

--
-- Name: positions_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.positions_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.positions_id_seq OWNER TO postgres;

--
-- Name: positions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.positions_id_seq OWNED BY public.positions.id;


--
-- Name: results; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.results (
    id integer NOT NULL,
    voter_id integer NOT NULL,
    position_id integer NOT NULL,
    nominee_id integer NOT NULL,
    election_id integer NOT NULL,
    voter_ip character varying(191),
    voter_code character varying(191),
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone
);


ALTER TABLE public.results OWNER TO postgres;

--
-- Name: results_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.results_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.results_id_seq OWNER TO postgres;

--
-- Name: results_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.results_id_seq OWNED BY public.results.id;


--
-- Name: slots; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.slots (
    id integer NOT NULL,
    position_id integer NOT NULL,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone
);


ALTER TABLE public.slots OWNER TO postgres;

--
-- Name: slots_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.slots_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.slots_id_seq OWNER TO postgres;

--
-- Name: slots_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.slots_id_seq OWNED BY public.slots.id;


--
-- Name: users; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.users (
    id integer NOT NULL,
    name character varying(191) NOT NULL,
    email character varying(191) NOT NULL,
    password character varying(191) NOT NULL,
    remember_token character varying(100),
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone
);


ALTER TABLE public.users OWNER TO postgres;

--
-- Name: users_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.users_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.users_id_seq OWNER TO postgres;

--
-- Name: users_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.users_id_seq OWNED BY public.users.id;


--
-- Name: voters; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.voters (
    id integer NOT NULL,
    email character varying(191),
    ip character varying(191),
    confirmation_token character varying(25),
    phone character varying(191),
    name character varying(191),
    member_id character varying(191),
    remember_token character varying(100),
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone
);


ALTER TABLE public.voters OWNER TO postgres;

--
-- Name: voters_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.voters_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.voters_id_seq OWNER TO postgres;

--
-- Name: voters_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.voters_id_seq OWNED BY public.voters.id;


--
-- Name: codes id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.codes ALTER COLUMN id SET DEFAULT nextval('public.codes_id_seq'::regclass);


--
-- Name: election_slot id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.election_slot ALTER COLUMN id SET DEFAULT nextval('public.election_slot_id_seq'::regclass);


--
-- Name: elections id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.elections ALTER COLUMN id SET DEFAULT nextval('public.elections_id_seq'::regclass);


--
-- Name: ipvalidations id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.ipvalidations ALTER COLUMN id SET DEFAULT nextval('public.ipvalidations_id_seq'::regclass);


--
-- Name: members id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.members ALTER COLUMN id SET DEFAULT nextval('public.members_id_seq'::regclass);


--
-- Name: migrations id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.migrations ALTER COLUMN id SET DEFAULT nextval('public.migrations_id_seq'::regclass);


--
-- Name: nominee_slot id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.nominee_slot ALTER COLUMN id SET DEFAULT nextval('public.nominee_slot_id_seq'::regclass);


--
-- Name: nominees id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.nominees ALTER COLUMN id SET DEFAULT nextval('public.nominees_id_seq'::regclass);


--
-- Name: positions id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.positions ALTER COLUMN id SET DEFAULT nextval('public.positions_id_seq'::regclass);


--
-- Name: results id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.results ALTER COLUMN id SET DEFAULT nextval('public.results_id_seq'::regclass);


--
-- Name: slots id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.slots ALTER COLUMN id SET DEFAULT nextval('public.slots_id_seq'::regclass);


--
-- Name: users id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users ALTER COLUMN id SET DEFAULT nextval('public.users_id_seq'::regclass);


--
-- Name: voters id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.voters ALTER COLUMN id SET DEFAULT nextval('public.voters_id_seq'::regclass);


--
-- Data for Name: codes; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.codes (id, code, is_used, created_at, updated_at) FROM stdin;
\.


--
-- Data for Name: election_slot; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.election_slot (id, slot_id, election_id, created_at, updated_at) FROM stdin;
1	1	1	\N	\N
2	2	1	\N	\N
3	3	1	\N	\N
4	4	1	\N	\N
5	5	1	\N	\N
6	6	1	\N	\N
7	7	2	\N	\N
8	8	2	\N	\N
9	9	2	\N	\N
10	10	2	\N	\N
11	11	2	\N	\N
12	12	2	\N	\N
13	13	3	\N	\N
14	14	3	\N	\N
15	15	3	\N	\N
16	16	3	\N	\N
17	17	3	\N	\N
18	18	3	\N	\N
19	19	4	\N	\N
20	20	4	\N	\N
21	21	4	\N	\N
22	22	4	\N	\N
23	23	4	\N	\N
24	24	4	\N	\N
25	25	5	\N	\N
26	26	5	\N	\N
27	27	5	\N	\N
28	28	5	\N	\N
29	29	5	\N	\N
30	30	5	\N	\N
\.


--
-- Data for Name: elections; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.elections (id, title, status, start, "end", can_see_result, created_at, updated_at) FROM stdin;
1	Polarised content-based support	1	2022-07-17 15:48:20	2022-08-17 18:53:14	f	2022-08-22 03:31:56	2022-08-22 03:31:56
2	Fully-configurable modular hardware	0	2022-06-13 19:07:03	2022-07-18 12:09:53	f	2022-08-22 03:31:56	2022-08-22 03:31:56
3	Upgradable eco-centric knowledgeuser	1	2022-06-11 02:05:00	2022-07-26 03:03:19	f	2022-08-22 03:31:56	2022-08-22 03:31:56
4	Future-proofed object-oriented function	0	2022-08-05 00:04:43	2022-08-10 22:12:06	f	2022-08-22 03:31:56	2022-08-22 03:31:56
5	Decentralized reciprocal portal	0	2022-05-25 17:11:42	2022-07-21 13:58:50	f	2022-08-22 03:31:56	2022-08-22 03:31:56
\.


--
-- Data for Name: ipvalidations; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.ipvalidations (id, ip, election_id, created_at, updated_at) FROM stdin;
\.


--
-- Data for Name: members; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.members (id, member_id, created_at, updated_at) FROM stdin;
\.


--
-- Data for Name: migrations; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.migrations (id, migration, batch) FROM stdin;
15	2014_10_12_000000_create_users_table	1
16	2014_10_12_100000_create_password_resets_table	1
17	2019_05_22_122741_create_elections_table	1
18	2019_05_22_122900_create_positions_table	1
19	2019_05_22_123116_create_nominees_table	1
20	2019_05_22_123153_create_results_table	1
21	2019_05_22_123734_create_voters_table	1
22	2019_05_22_162345_create_election_position_pivot_table	1
23	2019_05_22_185526_create_slots_table	1
24	2019_05_22_195503_create_nominee_slot_pivot_table	1
25	2019_05_22_195556_create_election_slot_pivot_table	1
26	2019_06_10_090553_create_ipvalidation_table	1
27	2020_11_26_050855_create_codes_table	1
28	2020_11_30_170026_create_members_table	1
\.


--
-- Data for Name: nominee_slot; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.nominee_slot (id, slot_id, nominee_id, created_at, updated_at) FROM stdin;
1	1	1	\N	\N
2	1	2	\N	\N
3	1	3	\N	\N
4	1	4	\N	\N
5	1	5	\N	\N
6	2	6	\N	\N
7	2	7	\N	\N
8	2	8	\N	\N
9	2	9	\N	\N
10	2	10	\N	\N
11	3	11	\N	\N
12	3	12	\N	\N
13	3	13	\N	\N
14	3	14	\N	\N
15	3	15	\N	\N
16	4	16	\N	\N
17	4	17	\N	\N
18	4	18	\N	\N
19	4	19	\N	\N
20	4	20	\N	\N
21	5	21	\N	\N
22	5	22	\N	\N
23	5	23	\N	\N
24	5	24	\N	\N
25	5	25	\N	\N
26	6	26	\N	\N
27	6	27	\N	\N
28	6	28	\N	\N
29	6	29	\N	\N
30	6	30	\N	\N
31	7	31	\N	\N
32	7	32	\N	\N
33	7	33	\N	\N
34	7	34	\N	\N
35	7	35	\N	\N
36	8	36	\N	\N
37	8	37	\N	\N
38	8	38	\N	\N
39	8	39	\N	\N
40	8	40	\N	\N
\.


--
-- Data for Name: nominees; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.nominees (id, first_name, last_name, email, image, description, created_at, updated_at) FROM stdin;
8	Lucas	Dickinson	morissette.rupert@example.net	http://placeimg.com/640/480/people	Laudantium corrupti consequatur enim id sed reiciendis exercitationem.	2022-08-22 03:31:57	2022-08-22 03:31:57
14	Audie	Dicki	bryce08@example.com	http://placeimg.com/640/480/people	Quasi quis dolore quia.	2022-08-22 03:31:57	2022-08-22 03:31:57
9	Keaton	Rau	herbert.grimes@example.com	http://placeimg.com/640/480/people	Suscipit ducimus tenetur autem magni.	2022-08-22 03:31:57	2022-08-22 03:31:57
10	Remington	Ankunding	finn54@example.org	http://placeimg.com/640/480/people	Et doloribus ex eos voluptate non molestiae.	2022-08-22 03:31:57	2022-08-22 03:31:57
11	Max	O'Reilly	dewayne.schimmel@example.net	http://placeimg.com/640/480/people	Minus dolorum qui sed velit doloribus.	2022-08-22 03:31:57	2022-08-22 03:31:57
12	Alfreda	Wyman	ledner.lane@example.com	http://placeimg.com/640/480/people	Facilis sed et odit consequatur labore.	2022-08-22 03:31:57	2022-08-22 03:31:57
13	Jermaine	Dach	khalid97@example.org	http://placeimg.com/640/480/people	Assumenda eum nostrum expedita similique quas maxime.	2022-08-22 03:31:57	2022-08-22 03:31:57
15	Renee	Moen	karley31@example.net	http://placeimg.com/640/480/people	Neque aperiam in consequuntur omnis quos amet nihil.	2022-08-22 03:31:57	2022-08-22 03:31:57
16	Wayne	Nolan	ternser@example.com	http://placeimg.com/640/480/people	Suscipit rerum distinctio et in distinctio alias nobis.	2022-08-22 03:31:57	2022-08-22 03:31:57
17	Sincere	McGlynn	neil.pouros@example.com	http://placeimg.com/640/480/people	Officia cumque quibusdam sed rem perferendis temporibus veniam.	2022-08-22 03:31:57	2022-08-22 03:31:57
18	Tressa	Steuber	hazle78@example.com	http://placeimg.com/640/480/people	Quia nesciunt aspernatur in delectus ipsam qui totam.	2022-08-22 03:31:57	2022-08-22 03:31:57
19	Verlie	Feeney	brook.bode@example.com	http://placeimg.com/640/480/people	Vel praesentium cupiditate quo aut.	2022-08-22 03:31:57	2022-08-22 03:31:57
20	Mariah	McDermott	violette42@example.com	http://placeimg.com/640/480/people	Officia sit vitae mollitia molestias.	2022-08-22 03:31:57	2022-08-22 03:31:57
21	Lulu	Heller	george.robel@example.net	http://placeimg.com/640/480/people	Atque facere voluptas molestiae.	2022-08-22 03:31:57	2022-08-22 03:31:57
22	Zena	Oberbrunner	lloyd23@example.com	http://placeimg.com/640/480/people	Deleniti vero unde voluptatem.	2022-08-22 03:31:57	2022-08-22 03:31:57
23	Therese	Gutkowski	claire.klocko@example.org	http://placeimg.com/640/480/people	In ea veritatis sed.	2022-08-22 03:31:57	2022-08-22 03:31:57
24	Brock	Larkin	mlesch@example.com	http://placeimg.com/640/480/people	Illum dolor doloremque magni eaque ab molestias ab.	2022-08-22 03:31:57	2022-08-22 03:31:57
25	Timmy	Keeling	wiegand.lucas@example.org	http://placeimg.com/640/480/people	Aut quia est atque ut.	2022-08-22 03:31:57	2022-08-22 03:31:57
26	Maggie	Ondricka	kurt.sipes@example.com	http://placeimg.com/640/480/people	Numquam ab est dolorem.	2022-08-22 03:31:57	2022-08-22 03:31:57
27	Jacklyn	Wyman	fae13@example.com	http://placeimg.com/640/480/people	Dolorum voluptatem voluptates aperiam rerum.	2022-08-22 03:31:57	2022-08-22 03:31:57
28	Judson	Cummings	morissette.jack@example.net	http://placeimg.com/640/480/people	Dolorem deleniti molestias voluptas provident voluptate suscipit alias.	2022-08-22 03:31:57	2022-08-22 03:31:57
29	Erich	Braun	leta53@example.com	http://placeimg.com/640/480/people	Iure saepe quos asperiores rerum ut rerum aut.	2022-08-22 03:31:57	2022-08-22 03:31:57
30	Thaddeus	Murphy	marquardt.audie@example.net	http://placeimg.com/640/480/people	Non maiores maiores quisquam vel quos.	2022-08-22 03:31:57	2022-08-22 03:31:57
31	Warren	Kulas	wuckert.sadye@example.org	http://placeimg.com/640/480/people	Omnis et ad vitae reprehenderit at.	2022-08-22 03:31:57	2022-08-22 03:31:57
32	Kylie	Thiel	hhill@example.org	http://placeimg.com/640/480/people	Dicta itaque soluta modi rerum.	2022-08-22 03:31:57	2022-08-22 03:31:57
33	Akeem	Larkin	pearlie.harber@example.net	http://placeimg.com/640/480/people	Nesciunt excepturi ut rerum reiciendis.	2022-08-22 03:31:57	2022-08-22 03:31:57
34	Camila	Schuppe	mariah.abbott@example.net	http://placeimg.com/640/480/people	Voluptatibus ducimus ab officia occaecati commodi.	2022-08-22 03:31:57	2022-08-22 03:31:57
35	Jessy	Funk	oharber@example.net	http://placeimg.com/640/480/people	Sint nostrum officiis saepe beatae.	2022-08-22 03:31:57	2022-08-22 03:31:57
36	Donna	Kovacek	wintheiser.briana@example.com	http://placeimg.com/640/480/people	Non et possimus iusto nesciunt fugit.	2022-08-22 03:31:57	2022-08-22 03:31:57
37	Daisy	McClure	viola26@example.org	http://placeimg.com/640/480/people	Quasi tempora consequatur accusamus id nisi eum.	2022-08-22 03:31:57	2022-08-22 03:31:57
38	Lavonne	Stark	white.may@example.net	http://placeimg.com/640/480/people	Soluta voluptatem quod sit libero eligendi voluptates sapiente aut.	2022-08-22 03:31:57	2022-08-22 03:31:57
1	Ashtynn	Lueilwitz	lea.pagac@example.net	nominee_64997kWGBEwsKjdgoNFaTbDDng8ISzWpjoDjC1NVc7EVV.jpg	Tempora magni tenetur debitis animi consequuntur placeat.	2022-08-22 03:31:57	2022-11-03 12:52:27
3	Baron	Hills	roel.labadie@example.com	nominee_09928YkdTEc97qg3T8zaf1uDzbgm7G1JFpW2X0lofhGbR.jpg	Ut quas a laborum accusamus.	2022-08-22 03:31:57	2022-11-03 19:25:14
42	Abi	Oladipo	daudonmail@mail.com	nominee_57134PhX844mkoRNMZJWK7kgM7VALNTiwKWvVobTgBOlq.png	Fit candidate for secretary	2022-11-04 05:25:12	2022-11-14 16:26:15
\.


--
-- Data for Name: password_resets; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.password_resets (email, token, created_at) FROM stdin;
\.


--
-- Data for Name: positions; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.positions (id, name, created_at, updated_at) FROM stdin;
1	soluta	2022-08-22 03:31:56	2022-08-22 03:31:56
2	accusantium	2022-08-22 03:31:56	2022-08-22 03:31:56
3	ab	2022-08-22 03:31:56	2022-08-22 03:31:56
4	voluptatem	2022-08-22 03:31:56	2022-08-22 03:31:56
5	quidem	2022-08-22 03:31:56	2022-08-22 03:31:56
6	cupiditate	2022-08-22 03:31:56	2022-08-22 03:31:56
7	ducimus	2022-08-22 03:31:56	2022-08-22 03:31:56
8	delectus	2022-08-22 03:31:56	2022-08-22 03:31:56
9	repellat	2022-08-22 03:31:56	2022-08-22 03:31:56
10	nemo	2022-08-22 03:31:56	2022-08-22 03:31:56
11	omnis	2022-08-22 03:31:56	2022-08-22 03:31:56
12	voluptas	2022-08-22 03:31:56	2022-08-22 03:31:56
13	fuga	2022-08-22 03:31:57	2022-08-22 03:31:57
14	aut	2022-08-22 03:31:57	2022-08-22 03:31:57
15	et	2022-08-22 03:31:57	2022-08-22 03:31:57
16	nam	2022-08-22 03:31:57	2022-08-22 03:31:57
17	autem	2022-08-22 03:31:57	2022-08-22 03:31:57
18	ut	2022-08-22 03:31:57	2022-08-22 03:31:57
19	blanditiis	2022-08-22 03:31:57	2022-08-22 03:31:57
20	praesentium	2022-08-22 03:31:57	2022-08-22 03:31:57
21	sit	2022-08-22 03:31:57	2022-08-22 03:31:57
22	atque	2022-08-22 03:31:57	2022-08-22 03:31:57
23	porro	2022-08-22 03:31:57	2022-08-22 03:31:57
24	numquam	2022-08-22 03:31:57	2022-08-22 03:31:57
25	rem	2022-08-22 03:31:57	2022-08-22 03:31:57
26	eos	2022-08-22 03:31:57	2022-08-22 03:31:57
27	molestiae	2022-08-22 03:31:57	2022-08-22 03:31:57
28	aspernatur	2022-08-22 03:31:57	2022-08-22 03:31:57
29	commodi	2022-08-22 03:31:57	2022-08-22 03:31:57
30	distinctio	2022-08-22 03:31:57	2022-08-22 03:31:57
31	reprehenderit	2022-08-22 03:31:57	2022-08-22 03:31:57
32	qui	2022-08-22 03:31:57	2022-08-22 03:31:57
33	aliquid	2022-08-22 03:31:57	2022-08-22 03:31:57
34	nisi	2022-08-22 03:31:57	2022-08-22 03:31:57
35	voluptates	2022-08-22 03:31:57	2022-08-22 03:31:57
36	ipsum	2022-08-22 03:31:57	2022-08-22 03:31:57
37	non	2022-08-22 03:31:57	2022-08-22 03:31:57
38	maxime	2022-08-22 03:31:57	2022-08-22 03:31:57
39	alias	2022-08-22 03:31:57	2022-08-22 03:31:57
40	magni	2022-08-22 03:31:57	2022-08-22 03:31:57
41	Secretary	2022-09-24 01:43:50	2022-09-24 01:43:50
42	President	2022-09-24 01:43:50	2022-09-24 01:43:50
43	Vice President	2022-09-24 01:43:50	2022-09-24 03:47:30
45	principal	2022-09-24 03:49:20	2022-09-24 03:49:20
46	Tresurear	2022-10-08 11:29:42	2022-11-14 07:16:38
\.


--
-- Data for Name: results; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.results (id, voter_id, position_id, nominee_id, election_id, voter_ip, voter_code, created_at, updated_at) FROM stdin;
1	1	1	13	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
2	2	1	29	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
3	3	1	2	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
4	4	1	13	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
5	5	1	17	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
6	6	1	30	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
7	7	1	17	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
8	8	1	5	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
9	9	1	22	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
10	10	1	31	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
11	11	1	9	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
12	12	1	18	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
13	13	1	2	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
14	14	1	12	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
15	15	1	38	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
16	16	1	5	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
17	17	1	27	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
18	18	1	7	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
19	19	1	22	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
20	20	1	36	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
21	21	1	2	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
22	22	1	3	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
23	23	1	20	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
24	24	1	4	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
25	25	1	29	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
26	26	1	1	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
27	27	1	36	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
28	28	1	36	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
29	29	1	21	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
30	30	1	35	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
31	31	1	28	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
32	32	1	6	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
33	33	1	8	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
34	34	1	4	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
35	35	1	39	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
36	36	1	37	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
37	37	1	22	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
38	38	1	18	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
39	39	1	31	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
40	40	1	6	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
41	41	1	34	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
42	42	1	14	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
43	43	1	13	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
44	44	1	4	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
45	45	1	13	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
46	46	1	6	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
47	47	1	12	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
48	48	1	20	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
49	49	1	5	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
50	50	1	37	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
51	51	1	4	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
52	52	1	11	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
53	53	1	18	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
54	54	1	39	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
55	55	1	33	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
56	56	1	20	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
57	57	1	40	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
58	58	1	11	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
59	59	1	14	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
60	60	1	27	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
61	61	1	28	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
62	62	1	9	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
63	63	1	13	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
64	64	1	22	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
65	65	1	40	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
66	66	1	36	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
67	67	1	22	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
68	68	1	15	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
69	69	1	15	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
70	70	1	36	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
71	1	2	38	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
72	2	2	16	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
73	3	2	33	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
74	4	2	23	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
75	5	2	30	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
76	6	2	8	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
77	7	2	16	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
78	8	2	7	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
79	9	2	40	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
80	10	2	33	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
81	11	2	20	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
82	12	2	27	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
83	13	2	38	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
84	14	2	30	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
85	15	2	21	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
86	16	2	11	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
87	17	2	29	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
88	18	2	21	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
89	19	2	6	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
90	20	2	25	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
91	21	2	35	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
92	22	2	35	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
93	23	2	35	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
94	24	2	14	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
95	25	2	12	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
96	26	2	21	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
97	27	2	11	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
98	28	2	37	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
99	29	2	5	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
100	30	2	7	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
101	31	2	13	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
102	32	2	11	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
103	33	2	3	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
104	34	2	7	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
105	35	2	13	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
106	36	2	14	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
107	37	2	24	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
108	38	2	17	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
109	39	2	9	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
110	40	2	39	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
111	41	2	15	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
112	42	2	18	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
113	43	2	36	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
114	44	2	39	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
115	45	2	11	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
116	46	2	12	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
117	47	2	7	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
118	48	2	30	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
119	49	2	34	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
120	50	2	9	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
121	51	2	4	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
122	52	2	40	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
123	53	2	30	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
124	54	2	31	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
125	55	2	22	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
126	56	2	34	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
127	57	2	3	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
128	58	2	35	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
129	59	2	27	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
130	60	2	37	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
131	61	2	26	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
132	62	2	13	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
133	63	2	6	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
134	64	2	12	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
135	65	2	14	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
136	66	2	39	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
137	67	2	1	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
138	68	2	9	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
139	69	2	30	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
140	70	2	16	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
141	1	3	28	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
142	2	3	31	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
143	3	3	40	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
144	4	3	34	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
145	5	3	33	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
146	6	3	17	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
147	7	3	29	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
148	8	3	24	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
149	9	3	1	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
150	10	3	26	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
151	11	3	18	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
152	12	3	15	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
153	13	3	33	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
154	14	3	28	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
155	15	3	38	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
156	16	3	22	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
157	17	3	13	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
158	18	3	7	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
159	19	3	21	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
160	20	3	3	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
161	21	3	39	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
162	22	3	28	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
163	23	3	9	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
164	24	3	18	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
165	25	3	31	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
166	26	3	6	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
167	27	3	12	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
168	28	3	9	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
169	29	3	7	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
170	30	3	31	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
171	31	3	35	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
172	32	3	21	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
173	33	3	12	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
174	34	3	16	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
175	35	3	24	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
176	36	3	30	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
177	37	3	29	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
178	38	3	10	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
179	39	3	30	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
180	40	3	20	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
181	41	3	11	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
182	42	3	33	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
183	43	3	6	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
184	44	3	35	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
185	45	3	9	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
186	46	3	32	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
187	47	3	25	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
188	48	3	1	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
189	49	3	37	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
190	50	3	27	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
191	51	3	40	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
192	52	3	26	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
193	53	3	34	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
194	54	3	15	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
195	55	3	37	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
196	56	3	34	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
197	57	3	27	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
198	58	3	15	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
199	59	3	2	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
200	60	3	10	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
201	61	3	7	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
202	62	3	6	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
203	63	3	25	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
204	64	3	19	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
205	65	3	34	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
206	66	3	26	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
207	67	3	16	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
208	68	3	32	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
209	69	3	19	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
210	70	3	13	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
211	1	4	8	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
212	2	4	12	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
213	3	4	34	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
214	4	4	4	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
215	5	4	19	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
216	6	4	16	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
217	7	4	3	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
218	8	4	40	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
219	9	4	1	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
220	10	4	35	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
221	11	4	8	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
222	12	4	6	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
223	13	4	38	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
224	14	4	13	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
225	15	4	26	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
226	16	4	27	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
227	17	4	3	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
228	18	4	19	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
229	19	4	9	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
230	20	4	39	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
231	21	4	4	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
232	22	4	38	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
233	23	4	6	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
234	24	4	20	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
235	25	4	1	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
236	26	4	36	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
237	27	4	33	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
238	28	4	22	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
239	29	4	9	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
240	30	4	3	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
241	31	4	21	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
242	32	4	39	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
243	33	4	35	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
244	34	4	34	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
245	35	4	25	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
246	36	4	4	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
247	37	4	36	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
248	38	4	6	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
249	39	4	26	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
250	40	4	4	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
251	41	4	23	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
252	42	4	23	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
253	43	4	21	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
254	44	4	7	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
255	45	4	22	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
256	46	4	35	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
257	47	4	24	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
258	48	4	23	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
259	49	4	5	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
260	50	4	25	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
261	51	4	31	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
262	52	4	32	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
263	53	4	8	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
264	54	4	9	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
265	55	4	14	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
266	56	4	32	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
267	57	4	29	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
268	58	4	18	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
269	59	4	19	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
270	60	4	33	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
271	61	4	24	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
272	62	4	22	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
273	63	4	40	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
274	64	4	10	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
275	65	4	35	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
276	66	4	14	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
277	67	4	38	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
278	68	4	27	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
279	69	4	21	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
280	70	4	14	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
281	1	5	27	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
282	2	5	38	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
283	3	5	24	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
284	4	5	33	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
285	5	5	40	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
286	6	5	31	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
287	7	5	8	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
288	8	5	23	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
289	9	5	10	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
290	10	5	16	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
291	11	5	32	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
292	12	5	32	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
293	13	5	8	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
294	14	5	7	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
295	15	5	10	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
296	16	5	22	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
297	17	5	4	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
298	18	5	40	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
299	19	5	13	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
300	20	5	19	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
301	21	5	36	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
302	22	5	17	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
303	23	5	6	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
304	24	5	25	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
305	25	5	5	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
306	26	5	27	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
307	27	5	27	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
308	28	5	21	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
309	29	5	23	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
310	30	5	9	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
311	31	5	8	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
312	32	5	23	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
313	33	5	3	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
314	34	5	4	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
315	35	5	1	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
316	36	5	4	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
317	37	5	21	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
318	38	5	28	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
319	39	5	33	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
320	40	5	27	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
321	41	5	18	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
322	42	5	15	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
323	43	5	21	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
324	44	5	18	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
325	45	5	22	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
326	46	5	6	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
327	47	5	32	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
328	48	5	31	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
329	49	5	9	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
330	50	5	2	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
331	51	5	27	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
332	52	5	37	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
333	53	5	38	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
334	54	5	9	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
335	55	5	17	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
336	56	5	35	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
337	57	5	1	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
338	58	5	6	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
339	59	5	40	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
340	60	5	15	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
341	61	5	13	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
342	62	5	29	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
343	63	5	26	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
344	64	5	3	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
345	65	5	11	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
346	66	5	29	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
347	67	5	24	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
348	68	5	9	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
349	69	5	1	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
350	70	5	14	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
351	1	6	2	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
352	2	6	23	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
353	3	6	11	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
354	4	6	3	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
355	5	6	8	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
356	6	6	7	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
357	7	6	20	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
358	8	6	37	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
359	9	6	25	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
360	10	6	28	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
361	11	6	1	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
362	12	6	8	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
363	13	6	39	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
364	14	6	34	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
365	15	6	14	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
366	16	6	9	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
367	17	6	3	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
368	18	6	12	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
369	19	6	1	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
370	20	6	26	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
371	21	6	36	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
372	22	6	5	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
373	23	6	28	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
374	24	6	15	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
375	25	6	35	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
376	26	6	28	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
377	27	6	36	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
378	28	6	28	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
379	29	6	38	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
380	30	6	11	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
381	31	6	17	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
382	32	6	1	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
383	33	6	32	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
384	34	6	8	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
385	35	6	12	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
386	36	6	16	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
387	37	6	40	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
388	38	6	28	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
389	39	6	15	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
390	40	6	1	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
391	41	6	37	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
392	42	6	6	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
393	43	6	17	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
394	44	6	38	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
395	45	6	23	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
396	46	6	30	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
397	47	6	21	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
398	48	6	39	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
399	49	6	9	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
400	50	6	35	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
401	51	6	9	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
402	52	6	9	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
403	53	6	29	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
404	54	6	19	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
405	55	6	32	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
406	56	6	2	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
407	57	6	35	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
408	58	6	19	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
409	59	6	40	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
410	60	6	8	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
411	61	6	14	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
412	62	6	15	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
413	63	6	32	3	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
414	64	6	22	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
415	65	6	28	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
416	66	6	4	5	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
417	67	6	13	1	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
418	68	6	19	4	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
419	69	6	28	2	\N	\N	2022-08-22 03:31:57	2022-08-22 03:31:57
420	70	6	8	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
421	1	7	32	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
422	2	7	25	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
423	3	7	31	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
424	4	7	37	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
425	5	7	6	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
426	6	7	13	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
427	7	7	29	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
428	8	7	2	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
429	9	7	29	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
430	10	7	22	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
431	11	7	6	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
432	12	7	28	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
433	13	7	31	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
434	14	7	19	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
435	15	7	16	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
436	16	7	31	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
437	17	7	21	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
438	18	7	3	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
439	19	7	37	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
440	20	7	25	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
441	21	7	25	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
442	22	7	10	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
443	23	7	29	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
444	24	7	13	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
445	25	7	39	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
446	26	7	6	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
447	27	7	37	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
448	28	7	18	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
449	29	7	6	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
450	30	7	20	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
451	31	7	12	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
452	32	7	14	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
453	33	7	30	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
454	34	7	17	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
455	35	7	23	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
456	36	7	14	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
457	37	7	39	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
458	38	7	34	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
459	39	7	39	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
460	40	7	30	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
461	41	7	15	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
462	42	7	35	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
463	43	7	7	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
464	44	7	2	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
465	45	7	27	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
466	46	7	31	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
467	47	7	23	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
468	48	7	21	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
469	49	7	20	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
470	50	7	38	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
471	51	7	29	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
472	52	7	20	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
473	53	7	24	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
474	54	7	4	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
475	55	7	6	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
476	56	7	19	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
477	57	7	24	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
478	58	7	27	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
479	59	7	38	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
480	60	7	16	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
481	61	7	13	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
482	62	7	22	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
483	63	7	20	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
484	64	7	2	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
485	65	7	26	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
486	66	7	28	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
487	67	7	2	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
488	68	7	10	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
489	69	7	9	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
490	70	7	11	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
491	1	8	19	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
492	2	8	39	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
493	3	8	19	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
494	4	8	14	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
495	5	8	15	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
496	6	8	4	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
497	7	8	15	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
498	8	8	15	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
499	9	8	30	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
500	10	8	16	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
501	11	8	37	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
502	12	8	21	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
503	13	8	20	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
504	14	8	33	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
505	15	8	17	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
506	16	8	30	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
507	17	8	2	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
508	18	8	32	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
509	19	8	27	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
510	20	8	1	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
511	21	8	7	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
512	22	8	10	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
513	23	8	4	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
514	24	8	37	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
515	25	8	29	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
516	26	8	4	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
517	27	8	32	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
518	28	8	29	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
519	29	8	19	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
520	30	8	22	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
521	31	8	35	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
522	32	8	31	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
523	33	8	12	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
524	34	8	23	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
525	35	8	25	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
526	36	8	20	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
527	37	8	11	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
528	38	8	19	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
529	39	8	25	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
530	40	8	16	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
531	41	8	33	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
532	42	8	6	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
533	43	8	32	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
534	44	8	40	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
535	45	8	23	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
536	46	8	22	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
537	47	8	12	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
538	48	8	30	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
539	49	8	20	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
540	50	8	7	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
541	51	8	11	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
542	52	8	27	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
543	53	8	7	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
544	54	8	26	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
545	55	8	22	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
546	56	8	4	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
547	57	8	22	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
548	58	8	35	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
549	59	8	7	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
550	60	8	27	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
551	61	8	1	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
552	62	8	22	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
553	63	8	3	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
554	64	8	31	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
555	65	8	25	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
556	66	8	28	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
557	67	8	17	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
558	68	8	6	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
559	69	8	7	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
560	70	8	17	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
561	1	9	3	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
562	2	9	5	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
563	3	9	2	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
564	4	9	30	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
565	5	9	6	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
566	6	9	3	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
567	7	9	38	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
568	8	9	21	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
569	9	9	36	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
570	10	9	14	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
571	11	9	22	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
572	12	9	8	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
573	13	9	40	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
574	14	9	18	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
575	15	9	36	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
576	16	9	3	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
577	17	9	19	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
578	18	9	25	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
579	19	9	36	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
580	20	9	22	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
581	21	9	9	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
582	22	9	22	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
583	23	9	24	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
584	24	9	31	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
585	25	9	21	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
586	26	9	37	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
587	27	9	33	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
588	28	9	6	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
589	29	9	5	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
590	30	9	26	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
591	31	9	32	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
592	32	9	34	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
593	33	9	21	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
594	34	9	33	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
595	35	9	6	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
596	36	9	5	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
597	37	9	17	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
598	38	9	5	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
599	39	9	21	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
600	40	9	2	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
601	41	9	11	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
602	42	9	33	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
603	43	9	22	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
604	44	9	13	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
605	45	9	27	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
606	46	9	33	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
607	47	9	23	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
608	48	9	26	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
609	49	9	21	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
610	50	9	35	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
611	51	9	27	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
612	52	9	8	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
613	53	9	39	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
614	54	9	4	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
615	55	9	22	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
616	56	9	39	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
617	57	9	22	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
618	58	9	39	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
619	59	9	7	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
620	60	9	16	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
621	61	9	24	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
622	62	9	21	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
623	63	9	26	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
624	64	9	3	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
625	65	9	18	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
626	66	9	19	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
627	67	9	19	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
628	68	9	5	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
629	69	9	29	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
630	70	9	19	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
631	1	10	26	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
632	2	10	26	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
633	3	10	23	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
634	4	10	1	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
635	5	10	40	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
636	6	10	32	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
637	7	10	16	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
638	8	10	21	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
639	9	10	38	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
640	10	10	3	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
641	11	10	16	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
642	12	10	23	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
643	13	10	12	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
644	14	10	18	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
645	15	10	31	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
646	16	10	39	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
647	17	10	28	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
648	18	10	26	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
649	19	10	1	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
650	20	10	29	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
651	21	10	39	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
652	22	10	19	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
653	23	10	16	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
654	24	10	7	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
655	25	10	27	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
656	26	10	29	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
657	27	10	5	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
658	28	10	9	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
659	29	10	14	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
660	30	10	19	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
661	31	10	17	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
662	32	10	22	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
663	33	10	9	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
664	34	10	23	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
665	35	10	37	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
666	36	10	12	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
667	37	10	19	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
668	38	10	27	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
669	39	10	24	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
670	40	10	27	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
671	41	10	4	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
672	42	10	29	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
673	43	10	29	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
674	44	10	15	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
675	45	10	15	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
676	46	10	21	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
677	47	10	1	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
678	48	10	37	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
679	49	10	19	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
680	50	10	8	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
681	51	10	7	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
682	52	10	11	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
683	53	10	33	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
684	54	10	8	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
685	55	10	3	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
686	56	10	6	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
687	57	10	5	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
688	58	10	23	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
689	59	10	10	3	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
690	60	10	35	2	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
691	61	10	21	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
692	62	10	31	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
693	63	10	14	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
694	64	10	8	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
695	65	10	11	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
696	66	10	27	1	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
697	67	10	38	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
698	68	10	39	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
699	69	10	9	4	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
700	70	10	5	5	\N	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
\.


--
-- Data for Name: slots; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.slots (id, position_id, created_at, updated_at) FROM stdin;
1	11	2022-08-22 03:31:57	2022-08-22 03:31:57
2	12	2022-08-22 03:31:57	2022-08-22 03:31:57
3	13	2022-08-22 03:31:57	2022-08-22 03:31:57
4	14	2022-08-22 03:31:57	2022-08-22 03:31:57
5	15	2022-08-22 03:31:57	2022-08-22 03:31:57
6	16	2022-08-22 03:31:57	2022-08-22 03:31:57
7	17	2022-08-22 03:31:57	2022-08-22 03:31:57
8	18	2022-08-22 03:31:57	2022-08-22 03:31:57
9	19	2022-08-22 03:31:57	2022-08-22 03:31:57
10	20	2022-08-22 03:31:57	2022-08-22 03:31:57
11	21	2022-08-22 03:31:57	2022-08-22 03:31:57
12	22	2022-08-22 03:31:57	2022-08-22 03:31:57
13	23	2022-08-22 03:31:57	2022-08-22 03:31:57
14	24	2022-08-22 03:31:57	2022-08-22 03:31:57
15	25	2022-08-22 03:31:57	2022-08-22 03:31:57
16	26	2022-08-22 03:31:57	2022-08-22 03:31:57
17	27	2022-08-22 03:31:57	2022-08-22 03:31:57
18	28	2022-08-22 03:31:57	2022-08-22 03:31:57
19	29	2022-08-22 03:31:57	2022-08-22 03:31:57
20	30	2022-08-22 03:31:57	2022-08-22 03:31:57
21	31	2022-08-22 03:31:57	2022-08-22 03:31:57
22	32	2022-08-22 03:31:57	2022-08-22 03:31:57
23	33	2022-08-22 03:31:57	2022-08-22 03:31:57
24	34	2022-08-22 03:31:57	2022-08-22 03:31:57
25	35	2022-08-22 03:31:57	2022-08-22 03:31:57
26	36	2022-08-22 03:31:57	2022-08-22 03:31:57
27	37	2022-08-22 03:31:57	2022-08-22 03:31:57
28	38	2022-08-22 03:31:57	2022-08-22 03:31:57
29	39	2022-08-22 03:31:57	2022-08-22 03:31:57
30	40	2022-08-22 03:31:57	2022-08-22 03:31:57
\.


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.users (id, name, email, password, remember_token, created_at, updated_at) FROM stdin;
1	Admin	admin@mail.com	$2y$10$3rhkkZaioFOrAPM/J4QXQ.p6AMMdnTX7Sh/GYHmcmB8DIK85F78KO	\N	2022-08-22 03:31:58	2022-08-22 03:31:58
3	test admin	admin@gmail.com	pbkdf2_sha256$320000$XHuXWO5orSKQ$a03T/vmqdipUGPgkOQjEmSfY7GC3HpYL5PEy1rqvtAA=	\N	2022-10-29 17:23:08	2022-10-29 17:23:08
\.


--
-- Data for Name: voters; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.voters (id, email, ip, confirmation_token, phone, name, member_id, remember_token, created_at, updated_at) FROM stdin;
1	jones.laisha@example.com	67.0.20.215	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
2	kwelch@example.com	188.158.203.2	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
3	kkeebler@example.net	5.255.101.1	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
4	ggrady@example.org	243.187.200.29	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
5	antonietta.collier@example.com	106.170.237.24	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
6	larkin.hailey@example.org	150.86.188.157	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
7	raphaelle.rodriguez@example.org	129.164.29.227	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
8	gerald.jerde@example.com	113.70.80.236	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
9	orrin49@example.com	16.3.221.157	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
10	dicki.patrick@example.com	220.83.33.51	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
11	jaskolski.darby@example.org	15.91.65.140	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
12	malinda21@example.org	121.169.222.47	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
13	ardella.schamberger@example.net	126.77.95.102	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
14	waters.timmy@example.com	143.25.216.91	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
15	estrella68@example.net	168.171.103.31	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
16	buckridge.eladio@example.org	88.167.251.71	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
17	pjaskolski@example.com	252.175.170.210	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
18	gyundt@example.org	210.79.129.40	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
19	jkovacek@example.net	208.127.143.158	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
20	haley.sanford@example.com	88.206.250.72	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
21	annabel.prohaska@example.com	159.4.11.242	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
22	ehodkiewicz@example.com	33.36.27.28	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
23	trystan.harber@example.org	52.75.36.150	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
24	gerhard.pagac@example.com	139.231.76.148	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
25	vkirlin@example.net	63.29.56.144	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
26	yost.patricia@example.org	180.20.249.216	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
27	connelly.alta@example.net	236.19.166.98	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
28	hills.zula@example.com	247.6.38.20	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
29	vonrueden.willa@example.net	31.98.188.213	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
30	thompson.ross@example.com	236.32.141.201	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
31	elmore.graham@example.org	235.88.201.228	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
32	clay67@example.net	16.218.8.110	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
33	eleanore02@example.org	241.183.46.220	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
34	rutherford.alice@example.org	156.85.51.135	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
35	lysanne.hessel@example.net	182.43.243.213	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
36	woodrow36@example.net	46.129.84.211	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
37	eduardo28@example.org	176.136.219.187	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
38	zgorczany@example.org	42.167.211.36	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
39	tsenger@example.com	102.5.117.145	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
40	rkling@example.net	250.253.233.222	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
41	oconnell.heath@example.com	190.253.56.143	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
42	kole.frami@example.org	214.185.126.243	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
43	lily62@example.com	181.91.210.158	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
44	lela.metz@example.org	189.131.177.183	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
45	vaughn19@example.com	104.219.1.144	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
46	rutherford.carter@example.com	27.143.244.216	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
47	norval88@example.net	252.171.252.89	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
48	kevon68@example.net	111.254.20.170	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
49	caterina.schuppe@example.com	103.69.51.228	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
50	kirlin.camden@example.org	89.76.161.145	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
51	zelda.kassulke@example.org	2.123.59.47	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
52	marks.philip@example.com	168.137.98.39	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
53	elittel@example.com	27.200.99.40	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
54	raina.price@example.com	71.110.212.240	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
55	kemmer.frederic@example.org	105.181.201.114	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
56	adam15@example.com	43.69.130.148	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
57	shyann07@example.net	26.86.85.104	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
58	alva.gleichner@example.org	32.175.5.14	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
59	russel00@example.net	190.233.247.100	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
60	mollie13@example.com	43.1.32.110	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
61	jaskolski.elvera@example.net	27.53.23.219	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
62	von.damian@example.com	99.232.213.217	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
63	luna.raynor@example.org	60.22.134.219	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
64	arlo82@example.net	36.5.26.75	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
65	lockman.jayme@example.net	225.45.251.224	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
66	jbarton@example.com	97.255.162.239	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
67	terry82@example.com	254.203.190.197	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
68	helen35@example.org	144.18.45.191	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
69	nicolas.lewis@example.org	249.56.48.33	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
70	lmitchell@example.com	112.253.200.216	\N	\N	\N	\N	\N	2022-08-22 03:31:56	2022-08-22 03:31:56
\.


--
-- Name: codes_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.codes_id_seq', 1, false);


--
-- Name: election_slot_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.election_slot_id_seq', 30, true);


--
-- Name: elections_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.elections_id_seq', 5, true);


--
-- Name: ipvalidations_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.ipvalidations_id_seq', 1, false);


--
-- Name: members_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.members_id_seq', 1, false);


--
-- Name: migrations_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.migrations_id_seq', 28, true);


--
-- Name: nominee_slot_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.nominee_slot_id_seq', 40, true);


--
-- Name: nominees_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.nominees_id_seq', 43, true);


--
-- Name: positions_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.positions_id_seq', 47, true);


--
-- Name: results_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.results_id_seq', 700, true);


--
-- Name: slots_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.slots_id_seq', 30, true);


--
-- Name: users_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.users_id_seq', 3, true);


--
-- Name: voters_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.voters_id_seq', 70, true);


--
-- Name: codes codes_code_unique; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.codes
    ADD CONSTRAINT codes_code_unique UNIQUE (code);


--
-- Name: codes codes_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.codes
    ADD CONSTRAINT codes_pkey PRIMARY KEY (id);


--
-- Name: election_slot election_slot_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.election_slot
    ADD CONSTRAINT election_slot_pkey PRIMARY KEY (id);


--
-- Name: elections elections_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.elections
    ADD CONSTRAINT elections_pkey PRIMARY KEY (id);


--
-- Name: ipvalidations ipvalidations_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.ipvalidations
    ADD CONSTRAINT ipvalidations_pkey PRIMARY KEY (id);


--
-- Name: members members_member_id_unique; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.members
    ADD CONSTRAINT members_member_id_unique UNIQUE (member_id);


--
-- Name: members members_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.members
    ADD CONSTRAINT members_pkey PRIMARY KEY (id);


--
-- Name: migrations migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.migrations
    ADD CONSTRAINT migrations_pkey PRIMARY KEY (id);


--
-- Name: nominee_slot nominee_slot_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.nominee_slot
    ADD CONSTRAINT nominee_slot_pkey PRIMARY KEY (id);


--
-- Name: nominees nominees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.nominees
    ADD CONSTRAINT nominees_pkey PRIMARY KEY (id);


--
-- Name: positions positions_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.positions
    ADD CONSTRAINT positions_pkey PRIMARY KEY (id);


--
-- Name: results results_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.results
    ADD CONSTRAINT results_pkey PRIMARY KEY (id);


--
-- Name: slots slots_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.slots
    ADD CONSTRAINT slots_pkey PRIMARY KEY (id);


--
-- Name: results uqe_vote; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.results
    ADD CONSTRAINT uqe_vote UNIQUE (voter_id, position_id, election_id);


--
-- Name: users users_email_unique; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_email_unique UNIQUE (email);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: voters voters_confirmation_token_unique; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.voters
    ADD CONSTRAINT voters_confirmation_token_unique UNIQUE (confirmation_token);


--
-- Name: voters voters_member_id_unique; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.voters
    ADD CONSTRAINT voters_member_id_unique UNIQUE (member_id);


--
-- Name: voters voters_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.voters
    ADD CONSTRAINT voters_pkey PRIMARY KEY (id);


--
-- Name: password_resets_email_index; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX password_resets_email_index ON public.password_resets USING btree (email);


--
-- PostgreSQL database dump complete
--

