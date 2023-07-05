/*
 Navicat Premium Data Transfer

 Source Server         : postgres
 Source Server Type    : PostgreSQL
 Source Server Version : 150003 (150003)
 Source Host           : localhost:5432
 Source Catalog        : database
 Source Schema         : public

 Target Server Type    : PostgreSQL
 Target Server Version : 150003 (150003)
 File Encoding         : 65001

 Date: 05/07/2023 17:50:12
*/


-- ----------------------------
-- Sequence structure for demo_user_id_seq
-- ----------------------------
DROP SEQUENCE IF EXISTS "public"."demo_user_id_seq";
CREATE SEQUENCE "public"."demo_user_id_seq" 
INCREMENT 1
MINVALUE  1
MAXVALUE 9223372036854775807
START 1
CACHE 1;
ALTER SEQUENCE "public"."demo_user_id_seq" OWNER TO "postgres";

-- ----------------------------
-- Table structure for demo_user
-- ----------------------------
DROP TABLE IF EXISTS "public"."demo_user";
CREATE TABLE "public"."demo_user" (
  "id" int8 NOT NULL DEFAULT nextval('demo_user_id_seq'::regclass),
  "username" varchar COLLATE "pg_catalog"."default" NOT NULL,
  "password" varchar COLLATE "pg_catalog"."default" NOT NULL,
  "role" int4 NOT NULL DEFAULT 0
)
;
ALTER TABLE "public"."demo_user" OWNER TO "postgres";

-- ----------------------------
-- Records of demo_user
-- ----------------------------
BEGIN;
INSERT INTO "public"."demo_user" ("id", "username", "password", "role") VALUES (2, 'USER1', 'PASS1', 0);
INSERT INTO "public"."demo_user" ("id", "username", "password", "role") VALUES (3, 'USER2', 'PASS2', 0);
INSERT INTO "public"."demo_user" ("id", "username", "password", "role") VALUES (4, 'USER3', 'PASS3', 0);
COMMIT;

-- ----------------------------
-- Alter sequences owned by
-- ----------------------------
ALTER SEQUENCE "public"."demo_user_id_seq"
OWNED BY "public"."demo_user"."id";
SELECT setval('"public"."demo_user_id_seq"', 1, true);

-- ----------------------------
-- Primary Key structure for table demo_user
-- ----------------------------
ALTER TABLE "public"."demo_user" ADD CONSTRAINT "demo_user_pkey" PRIMARY KEY ("id");
