-- MySQL dump 10.13  Distrib 8.0.42, for Win64 (x86_64)
--
-- Host: 127.0.0.1    Database: qiluoopen
-- ------------------------------------------------------
-- Server version	5.7.44-log

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!50503 SET NAMES utf8 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `seaql_migrations`
--

DROP TABLE IF EXISTS `seaql_migrations`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `seaql_migrations` (
  `version` varchar(255) NOT NULL,
  `applied_at` bigint(20) NOT NULL,
  PRIMARY KEY (`version`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `seaql_migrations`
--

LOCK TABLES `seaql_migrations` WRITE;
/*!40000 ALTER TABLE `seaql_migrations` DISABLE KEYS */;
INSERT INTO `seaql_migrations` VALUES ('m20220101_000001_create_table',1723226505);
/*!40000 ALTER TABLE `seaql_migrations` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_api_permission`
--

DROP TABLE IF EXISTS `sys_api_permission`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_api_permission` (
  `id` bigint(20) NOT NULL,
  `api` char(255) NOT NULL,
  `method` char(10) NOT NULL,
  `apiname` char(20) NOT NULL,
  `logcache` char(1) NOT NULL DEFAULT '0',
  `remark` varchar(512) DEFAULT NULL,
  `sort` int(11) NOT NULL,
  `created_at` datetime NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `api` (`api`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_api_permission`
--

LOCK TABLES `sys_api_permission` WRITE;
/*!40000 ALTER TABLE `sys_api_permission` DISABLE KEYS */;
INSERT INTO `sys_api_permission` VALUES (7232752187397280768,'/sys/job/add','Post','添加定时任务','0','但凡',1,'2024-08-23 22:15:00'),(7232752187535692800,'/sys/job/validate_cron','Post','验证cron表达式','0',NULL,3,'2024-08-23 22:15:00'),(7232752187606995968,'/sys/job/list','Get','获取定时任务列表','0',NULL,3,'2024-08-23 22:15:00'),(7232752187674104832,'/sys/job/edit','Put','编辑定时任务','0',NULL,4,'2024-08-23 22:15:00'),(7232752187741213696,'/sys/job/del','Delete','删除定时任务','0',NULL,5,'2024-08-23 22:15:00'),(7232752187812516864,'/sys/roleapi/add_many_role_api_transfer','Get','添加多个角色api','0',NULL,6,'2024-08-23 22:15:00'),(7232752187879625728,'/sys/roleapi/del','Delete','删除角色api','0',NULL,7,'2024-08-23 22:15:00'),(7232752187946734592,'/sys/roleapi/role_api_transfer_list','Get','获取角色api所有的选择列表','0',NULL,8,'2024-08-23 22:15:00'),(7232752188013843456,'/sys/roleapi/list','Get','获取角色api列表','0',NULL,9,'2024-08-23 22:15:00'),(7232752188085146624,'/sys/roleapi/edit','Put','编辑角色api','0',NULL,10,'2024-08-23 22:15:00'),(7232752188156449792,'/sys/roleapi/role_permission_list','Get','根据角色id获取api权限列表','0',NULL,11,'2024-08-23 22:15:00'),(7232752188219364352,'/sys/roleapi/add','Post','添加角色api','0',NULL,12,'2024-08-23 22:15:00'),(7232752188282278912,'/sys/dicttype/list','Get','获取字典类型列表','0',NULL,13,'2024-08-23 22:15:00'),(7232752188349387776,'/sys/dicttype/add','Post','添加字典类型','0',NULL,14,'2024-08-23 22:15:00'),(7232752188412302336,'/sys/dicttype/del','Delete','删除字典类型','0',NULL,15,'2024-08-23 22:15:00'),(7232752188479411200,'/sys/dicttype/edit','Put','编辑字典类型','0',NULL,16,'2024-08-23 22:15:00'),(7232752188546520064,'/sys/menu/edit','Put','编辑菜单','0',NULL,17,'2024-08-23 22:15:00'),(7232752188617823232,'/sys/menu/list','Get','菜单列表','0',NULL,18,'2024-08-23 22:15:00'),(7232752188697515008,'/sys/menu/add','Post','添加菜单','0',NULL,19,'2024-08-23 22:15:00'),(7232752188764623872,'/sys/menu/del','Delete','删除菜单','0',NULL,20,'2024-08-23 22:15:00'),(7232752188835927040,'/sys/menu/all_router','Get','全部路由','0',NULL,21,'2024-08-23 22:15:00'),(7232752188898841600,'/sys/menu/tree','Get','获取菜单树','0',NULL,22,'2024-08-23 22:15:00'),(7232752188965950464,'/sys/serverinfo/server_update','Get','更新服务器信息','0',NULL,23,'2024-08-23 22:15:00'),(7232752189028865024,'/sys/jobinfo/add','Post','添加定时任务日志','0',NULL,24,'2024-08-23 22:15:00'),(7232752189095973888,'/sys/jobinfo/list','Get','获取定时任务日志列表','0',NULL,25,'2024-08-23 22:15:00'),(7232752189158888448,'/sys/dictdata/get_by_type','Get','根据类型获取字典数据','0',NULL,26,'2024-08-23 22:15:00'),(7232752189221803008,'/sys/dictdata/list','Get','获取字典数据列表','0',NULL,27,'2024-08-23 22:15:00'),(7232752189280523264,'/sys/dictdata/del','Delete','删除字典数据','0',NULL,28,'2024-08-23 22:15:00'),(7232752189347632128,'/sys/dictdata/edit','Put','编辑字典数据','0',NULL,29,'2024-08-23 22:15:00'),(7232752189410546688,'/sys/dictdata/add','Post','添加字典数据','0',NULL,30,'2024-08-23 22:15:00'),(7232752189473461248,'/sys/dashboard/add','Post','添加仪表盘','0',NULL,31,'2024-08-23 22:15:00'),(7232752189557347328,'/sys/dashboard/list','Get','获取仪表盘列表','0',NULL,32,'2024-08-23 22:15:00'),(7232752189628650496,'/sys/role/list','Get','获取角色列表','0',NULL,33,'2024-08-23 22:15:00'),(7232752189691565056,'/sys/role/get_role_menus','Get','获取角色菜单','0',NULL,34,'2024-08-23 22:15:00'),(7232752189758673920,'/sys/role/menu','Get','获取角色菜单','0',NULL,35,'2024-08-23 22:15:00'),(7232752189821588480,'/sys/role/edit','Put','编辑角色','0',NULL,36,'2024-08-23 22:15:00'),(7232752189888697344,'/sys/role/tree','Get','获取角色树','0',NULL,37,'2024-08-23 22:15:00'),(7232752189951611904,'/sys/role/add','Post','添加角色','0',NULL,38,'2024-08-23 22:15:00'),(7232752190018720768,'/sys/role/del','Delete','删除角色','0',NULL,39,'2024-08-23 22:15:00'),(7232752190081635328,'/sys/apipermission/list','Get','获取api权限列表','0',NULL,40,'2024-08-23 22:15:00'),(7232752190144549888,'/sys/user/reset_password','Put','重置密码','0',NULL,41,'2024-08-23 22:15:00'),(7232752190207464448,'/sys/user/userinfo','Get','获取用户信息','0',NULL,42,'2024-08-23 22:15:00'),(7232752190270379008,'/sys/user/depts_roles','Get','获取用户拥有的用户部门和用户角色','0',NULL,43,'2024-08-23 22:15:00'),(7232752190333293568,'/sys/user/del','Delete','删除用户','0',NULL,44,'2024-08-23 22:15:00'),(7232752190396208128,'/sys/user/list','Get','获取用户列表','0',NULL,45,'2024-08-23 22:15:00'),(7232752190459122688,'/sys/user/add','Post','添加用户','0',NULL,46,'2024-08-23 22:15:00'),(7232752190526231552,'/sys/user/change_password','Put','修改密码','0',NULL,47,'2024-08-23 22:15:00'),(7232752190589146112,'/sys/user/edit','Put','编辑用户','0',NULL,48,'2024-08-23 22:15:00'),(7232752190656254976,'/sys/dept/edit','Put','编辑部门','0',NULL,49,'2024-08-23 22:15:00'),(7232752190727558144,'/sys/dept/add','Post','添加部门','0',NULL,50,'2024-08-23 22:15:00'),(7232752190794667008,'/sys/dept/list','Get','获取部门列表','0',NULL,51,'2024-08-23 22:15:00'),(7232752190861775872,'/sys/dept/del','Delete','删除部门','0',NULL,52,'2024-08-23 22:15:00'),(7232752190933079040,'/sys/dept/dept_tree','Get','获取部门树','0',NULL,53,'2024-08-23 22:15:00'),(7232752190995993600,'/sys/useronline/list','Get','获取在线用户列表','0',NULL,54,'2024-08-23 22:15:00'),(7232752191063102464,'/sys/logininfo/list','Get','获取登录日志列表','0',NULL,55,'2024-08-23 22:15:00'),(7232781502121939968,'/sys/user/delusers','Delete','删除用户','0',NULL,25,'2024-08-24 00:11:29'),(7232794958103483392,'/sys/dashboard/analysis/weeklyUserActivity','Get','获取仪表盘数据','0',NULL,24,'2024-08-24 01:04:57'),(7232794958170592256,'/sys/dashboard/analysis/total','Get','获取仪表盘数据','0',NULL,25,'2024-08-24 01:04:57'),(7232794958229312512,'/sys/dashboard/analysis/monthlySales','Get','获取仪表盘数据','0',NULL,26,'2024-08-24 01:04:57'),(7232794958279644160,'/sys/dashboard/analysis/userAccessSource','Get','获取仪表盘数据','0',NULL,27,'2024-08-24 01:04:57'),(7232794958317392896,'/sys/dashboard/workplace/dynamic','Get','获取仪表盘数据','0',NULL,28,'2024-08-24 01:04:57'),(7232794958363530240,'/sys/dashboard/workplace/total','Get','获取仪表盘数据','0',NULL,29,'2024-08-24 01:04:57'),(7232794958401278976,'/sys/dashboard/workplace/radar','Get','获取仪表盘数据','0',NULL,30,'2024-08-24 01:04:57'),(7232794958447416320,'/sys/dashboard/workplace/project','Get','获取仪表盘数据','0',NULL,31,'2024-08-24 01:04:57'),(7232794958493553664,'/sys/dashboard/workplace/team','Get','获取仪表盘数据','0',NULL,32,'2024-08-24 01:04:57'),(7233868069737501696,'/sys/apipermission/edit','Put','编辑api权限','0',NULL,63,'2024-08-27 00:09:07'),(7234253747373642752,'/sys/operationlog/list','Get','获取操作日志列表','0',NULL,52,'2024-08-28 01:41:39'),(7234570580194661376,'/sys/useronline/del','Delete','退出在线用户','0',NULL,47,'2024-08-28 22:40:38'),(7234616780964926464,'/sys/cache/clear','Post','清空缓存','0',NULL,18,'2024-08-29 01:44:13'),(7234616781027841024,'/sys/cache/list','Get','获取缓存列表','0',NULL,19,'2024-08-29 01:44:13'),(7235633673968456704,'/sys/role/get_role_depts','Get','获取角色部门','0',NULL,12,'2024-08-31 21:04:59'),(7235678367742071808,'/sys/job/hand_execute_job','Post','执行定时任务','0',NULL,67,'2024-09-01 00:02:35'),(7236784152022782976,'/sys/user/fresh_token','Put','刷新token','0',NULL,36,'2024-09-04 01:16:35'),(7236784153973134336,'/sys/user/update_avatar','Put','更新头像','0',NULL,38,'2024-09-04 01:16:35'),(7240020747752477696,'/sys/user/refersh_token','Put','刷新token','0',NULL,66,'2024-09-12 23:37:39'),(7241490807281062912,'/sys/user/login_out','Get','用户退出','0',NULL,66,'2024-09-17 00:59:09'),(7252729164858299392,'/sys/role/user_role_name_list','Get','获取用户拥有的角色名称列表','0',NULL,14,'2024-10-18 01:16:22'),(7252738573697192960,'/sys/dept/user_dept_name_list','Get','获取用户拥有的部门名称列表','0',NULL,44,'2024-10-18 01:53:45'),(7253067149336286208,'/sys/user/update_role_or_dept','Post','更新用户拥有的用户部门和用户角色','0',NULL,57,'2024-10-18 23:39:24'),(7348333842530210816,'/wechat/wxaccounts/del','Delete','删除WxAccounts','0',NULL,76,'2025-07-08 20:55:13'),(7348333842559570944,'/wechat/wxaccounts/edit','Put','编辑WxAccounts','0',NULL,77,'2025-07-08 20:55:14'),(7348333842567959552,'/wechat/wxaccounts/list','Get','获取WxAccounts列表','0',NULL,78,'2025-07-08 20:55:14'),(7348333842572153856,'/wechat/wxaccounts/add','Post','添加WxAccounts','0',NULL,79,'2025-07-08 20:55:14'),(7348333842584736768,'/wechat/wxautoreplies/add','Post','添加WxAutoReplies','0',NULL,80,'2025-07-08 20:55:14'),(7348333842609902592,'/wechat/wxautoreplies/del','Delete','删除WxAutoReplies','0',NULL,81,'2025-07-08 20:55:14'),(7348333842635068416,'/wechat/wxautoreplies/list','Get','获取WxAutoReplies列表','0',NULL,82,'2025-07-08 20:55:14'),(7348333843008361472,'/wechat/wxautoreplies/edit','Put','编辑WxAutoReplies','0',NULL,83,'2025-07-08 20:55:14'),(7348333843016750080,'/wechat/wxmenus/list','Get','获取WxMenus列表','0',NULL,84,'2025-07-08 20:55:14'),(7348333843071276032,'/wechat/wxmenus/del','Delete','删除WxMenus','0',NULL,85,'2025-07-08 20:55:14'),(7348333843075470336,'/wechat/wxmenus/pull_menu','Get','创建自定义菜单','0',NULL,86,'2025-07-08 20:55:14'),(7348333843092247552,'/wechat/wxmenus/add','Post','添加WxMenus','0',NULL,87,'2025-07-08 20:55:14'),(7348333843100636160,'/wechat/wxmenus/edit','Put','编辑WxMenus','0',NULL,88,'2025-07-08 20:55:14'),(7348333843104830464,'/wechat/wxmessages/list','Get','获取WxMessages列表','0',NULL,89,'2025-07-08 20:55:14'),(7348333843125801984,'/wechat/wxmessages/edit','Put','编辑WxMessages','0',NULL,90,'2025-07-08 20:55:14'),(7348333843138384896,'/wechat/wxmessages/del','Delete','删除WxMessages','0',NULL,91,'2025-07-08 20:55:14'),(7348333843150967808,'/wechat/wxmessages/add','Post','添加WxMessages','0',NULL,92,'2025-07-08 20:55:14'),(7348333843159356416,'/wechat/wxusers/add','Post','添加WxUsers','0',NULL,93,'2025-07-08 20:55:14'),(7348333843167745024,'/wechat/wxusers/edit','Put','编辑WxUsers','0',NULL,94,'2025-07-08 20:55:14'),(7348333843180327936,'/wechat/wxusers/del','Delete','删除WxUsers','0',NULL,95,'2025-07-08 20:55:14'),(7348333843184522240,'/wechat/wxusers/list','Get','获取WxUsers列表','0',NULL,96,'2025-07-08 20:55:14'),(7385368752226735104,'/test/test_api/list','Get','获取列表','0',NULL,75,'2025-10-19 01:38:44'),(7385368752235123712,'/test/test_api/del','Delete','删除TestApi','0',NULL,76,'2025-10-19 01:38:44'),(7385368752239318016,'/test/test_api/add','Post','添加TestApi','0',NULL,77,'2025-10-19 01:38:44'),(7385368752243512320,'/test/test_api/edit','Put','编辑TestApi','0',NULL,78,'2025-10-19 01:38:44'),(7401293722618336256,'/test/test_api/db_read_write_test','Put','db_read_write_test','0',NULL,76,'2025-12-02 00:18:53'),(7401293722630919168,'/test/test_api/db_name_test','Put','db_name_test','0',NULL,77,'2025-12-02 00:18:53'),(7401293722647696384,'/test/test_api/db_index_test','Put','db_index_test','0',NULL,81,'2025-12-02 00:18:53'),(7401293722651890688,'/test/test_api/db_auto_test','Put','db_auto_test','0',NULL,82,'2025-12-02 00:18:53'),(7402694868939478016,'/test/test_data_scope/del','Delete','Delete','0',NULL,83,'2025-12-05 21:06:32'),(7402694868947866624,'/test/test_data_scope/list','Get','list','0',NULL,84,'2025-12-05 21:06:32'),(7402694868947866625,'/test/test_data_scope/add','Post','add','0',NULL,85,'2025-12-05 21:06:32'),(7402694868952060928,'/test/test_data_scope/edit','Put','edit','0',NULL,86,'2025-12-05 21:06:32');
/*!40000 ALTER TABLE `sys_api_permission` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_dept`
--

DROP TABLE IF EXISTS `sys_dept`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_dept` (
  `dept_id` bigint(20) NOT NULL COMMENT '部门id',
  `parent_id` bigint(20) NOT NULL DEFAULT '0' COMMENT '父部门id',
  `dept_name` varchar(30) DEFAULT '' COMMENT '部门名称',
  `lft` int(11) NOT NULL DEFAULT '0',
  `rgt` int(11) NOT NULL DEFAULT '0',
  `depth` int(11) NOT NULL DEFAULT '0',
  `dept_category` varchar(100) DEFAULT NULL COMMENT '部门类别编码',
  `order` int(4) NOT NULL DEFAULT '0' COMMENT '显示顺序',
  `leader` bigint(20) DEFAULT NULL COMMENT '负责人',
  `phone` varchar(11) DEFAULT NULL COMMENT '联系电话',
  `email` varchar(50) DEFAULT NULL COMMENT '邮箱',
  `status` char(1) NOT NULL DEFAULT '0' COMMENT '部门状态（0正常 1停用）',
  `remark` varchar(500) DEFAULT NULL,
  `created_at` datetime DEFAULT NULL COMMENT '创建时间',
  `updated_at` datetime DEFAULT NULL COMMENT '更新时间',
  `deleted_at` datetime DEFAULT NULL,
  PRIMARY KEY (`dept_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='部门表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_dept`
--

LOCK TABLES `sys_dept` WRITE;
/*!40000 ALTER TABLE `sys_dept` DISABLE KEYS */;
INSERT INTO `sys_dept` VALUES (100,0,'科技',1,26,1,NULL,1,NULL,'15888888888','xxx@qq.com','0',NULL,'2024-07-03 01:17:31',NULL,NULL),(101,100,'深圳总公司',2,13,2,NULL,1,NULL,'15888888888','xxx@qq.com','0',NULL,'2024-07-03 01:17:31',NULL,NULL),(102,100,'长沙分公司',14,21,2,NULL,2,NULL,'187554971','xxx@qq.com','0','长沙的公司','2024-07-03 01:17:31',NULL,NULL),(103,101,'研发部门',3,4,3,NULL,1,1,'15888888888','xxx@qq.com','0',NULL,'2024-07-03 01:17:31',NULL,NULL),(104,101,'市场部门',5,6,3,NULL,2,NULL,'15888888888','xxx@qq.com','0',NULL,'2024-07-03 01:17:31',NULL,NULL),(105,101,'测试部门',7,8,3,NULL,3,NULL,'15888888888','xxx@qq.com','0',NULL,'2024-07-03 01:17:31',NULL,NULL),(106,101,'财务部门',9,10,3,NULL,4,NULL,'15888888888','xxx@qq.com','0',NULL,'2024-07-03 01:17:31',NULL,NULL),(107,101,'运维部门',11,12,3,NULL,5,NULL,'15888888888','xxx@qq.com','0',NULL,'2024-07-03 01:17:31',NULL,NULL),(108,102,'市场部门',15,16,3,NULL,1,NULL,'15888888888','xxx@qq.com','0',NULL,'2024-07-03 01:17:31',NULL,NULL),(109,102,'财务部门',17,18,3,NULL,2,NULL,'15888888888','xxx@qq.com','0',NULL,'2024-07-03 01:17:31',NULL,NULL),(7223731313261547520,102,'运维部门',19,20,3,NULL,3,NULL,NULL,NULL,'0','长沙运维部门',NULL,NULL,NULL),(7223731922878468096,100,'广州分公司',22,25,2,NULL,5,NULL,NULL,NULL,'0',NULL,NULL,NULL,NULL),(7223732104475054080,7223731922878468096,'小部门',23,24,3,NULL,3,NULL,NULL,NULL,'0',NULL,NULL,NULL,NULL);
/*!40000 ALTER TABLE `sys_dept` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_dict_data`
--

DROP TABLE IF EXISTS `sys_dict_data`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_dict_data` (
  `dict_code` bigint(20) NOT NULL COMMENT '字典编码',
  `dict_sort` int(4) NOT NULL DEFAULT '0' COMMENT '字典排序',
  `dict_label` varchar(100) NOT NULL DEFAULT '' COMMENT '字典标签',
  `dict_value` varchar(100) NOT NULL DEFAULT '' COMMENT '字典键值',
  `dict_type_id` bigint(20) NOT NULL COMMENT '字典类型',
  `created_at` datetime DEFAULT NULL COMMENT '创建时间',
  `updated_at` datetime DEFAULT NULL COMMENT '更新时间',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  PRIMARY KEY (`dict_code`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='字典数据表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_dict_data`
--

LOCK TABLES `sys_dict_data` WRITE;
/*!40000 ALTER TABLE `sys_dict_data` DISABLE KEYS */;
INSERT INTO `sys_dict_data` VALUES (1,1,'男','0',1,'2024-07-03 01:17:32',NULL,'性别男'),(2,2,'女','1',1,'2024-07-03 01:17:32',NULL,'性别女'),(3,3,'未知','2',1,'2024-07-03 01:17:32',NULL,'性别未知'),(4,1,'显示','0',2,'2024-07-03 01:17:32',NULL,'显示菜单a'),(5,2,'隐藏','1',2,'2024-07-03 01:17:32',NULL,'隐藏菜单'),(6,1,'正常','0',3,'2024-07-03 01:17:33',NULL,'正常状态'),(7,2,'停用','1',3,'2024-07-03 01:17:33',NULL,'停用状态'),(12,3,'是','Y',6,'2024-07-03 01:17:33',NULL,'系统默认是'),(13,4,'否','N',6,'2024-07-03 01:17:33',NULL,'系统默认否'),(14,1,'通知','1',7,'2024-07-03 01:17:33',NULL,'通知'),(15,2,'公告','2',7,'2024-07-03 01:17:33',NULL,'公告'),(16,1,'正常','0',8,'2024-07-03 01:17:33',NULL,'正常状态'),(17,2,'关闭','1',8,'2024-07-03 01:17:33',NULL,'关闭状态'),(18,1,'新增','1',9,'2024-07-03 01:17:33',NULL,'新增操作'),(19,2,'修改','2',9,'2024-07-03 01:17:33',NULL,'修改操作'),(20,3,'删除','3',9,'2024-07-03 01:17:33',NULL,'删除操作'),(21,4,'授权','4',9,'2024-07-03 01:17:33',NULL,'授权操作'),(22,5,'导出','5',9,'2024-07-03 01:17:33',NULL,'导出操作'),(23,6,'导入','6',9,'2024-07-03 01:17:33',NULL,'导入操作'),(24,7,'强退','7',9,'2024-07-03 01:17:33',NULL,'强退操作'),(25,8,'生成代码','8',9,'2024-07-03 01:17:33',NULL,'生成操作'),(26,9,'清空数据','9',9,'2024-07-03 01:17:33',NULL,'清空操作'),(27,1,'成功','0',10,'2024-07-03 01:17:33',NULL,'正常状态'),(28,2,'失败','1',10,'2024-07-03 01:17:33',NULL,'停用状态'),(29,99,'其他','0',9,'2024-07-03 01:17:33',NULL,'其他操作'),(30,0,'密码认证','password',11,'2024-07-03 01:17:33',NULL,'密码认证'),(31,0,'短信认证','sms',11,'2024-07-03 01:17:33',NULL,'短信认证'),(32,0,'邮件认证','email',11,'2024-07-03 01:17:33',NULL,'邮件认证'),(33,0,'小程序认证','xcx',11,'2024-07-03 01:17:33',NULL,'小程序认证'),(34,0,'三方登录认证','social',11,'2024-07-03 01:17:33',NULL,'三方登录认证'),(35,0,'PC','pc',12,'2024-07-03 01:17:33',NULL,'PC'),(36,0,'安卓','android',12,'2024-07-03 01:17:33',NULL,'安卓'),(37,0,'iOS','ios',12,'2024-07-03 01:17:33',NULL,'iOS'),(38,0,'小程序','xcx',12,'2024-07-03 01:17:33',NULL,'小程序'),(7229170738212769792,1,'请求网址','geturl',7229170480917385217,NULL,NULL,'GET请求网址'),(7229171315483217920,3,'发送邮件','sendemail',7229170480917385217,NULL,NULL,'发送邮件'),(7230797167031619584,1,'默认','default',7230795611045498880,NULL,NULL,NULL),(7230797303799484416,3,'系统','system',7230795611045498880,NULL,NULL,NULL),(7233442254918947840,1,'不记录','0',7233442155891430400,NULL,NULL,NULL),(7233442310988403712,3,'数据库','2',7233442155891430400,NULL,NULL,NULL),(7233442350909789184,2,'文件记录','1',7233442155891430400,NULL,NULL,NULL),(7233442400423547904,5,'数据库和文件','3',7233442155891430400,NULL,NULL,NULL),(7235988025748919296,3,'调用方法','invokefunction',7229170480917385217,NULL,NULL,'调用方法');
/*!40000 ALTER TABLE `sys_dict_data` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_dict_type`
--

DROP TABLE IF EXISTS `sys_dict_type`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_dict_type` (
  `dict_id` bigint(20) NOT NULL COMMENT '字典主键',
  `dict_name` varchar(100) NOT NULL COMMENT '字典名称',
  `dict_type` varchar(100) NOT NULL COMMENT '字典类型',
  `order` int(4) NOT NULL COMMENT '排序',
  `created_at` datetime NOT NULL COMMENT '创建时间',
  `updated_at` datetime NOT NULL COMMENT '更新时间',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  PRIMARY KEY (`dict_id`),
  UNIQUE KEY `tenant_id` (`dict_type`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='字典类型表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_dict_type`
--

LOCK TABLES `sys_dict_type` WRITE;
/*!40000 ALTER TABLE `sys_dict_type` DISABLE KEYS */;
INSERT INTO `sys_dict_type` VALUES (1,'用户性别','sys_user_sex',103,'2024-07-03 01:17:32','2024-07-03 01:17:32','用户性别列表'),(2,'菜单状态','sys_show_hide',103,'2024-07-03 01:17:32','2024-07-03 01:17:32','菜单状态列表'),(3,'系统开关','sys_normal_disable',103,'2024-07-03 01:17:32','2024-07-03 01:17:32','系统开关列表'),(6,'系统是否','sys_yes_no',103,'2024-07-03 01:17:32','2024-07-03 01:17:32','系统是否列表'),(7,'通知类型','sys_notice_type',103,'2024-07-03 01:17:32','2024-08-14 01:06:32','通知类型列表'),(8,'通知状态','sys_notice_status',103,'2024-07-03 01:17:32','2024-07-03 01:17:32','通知状态列表'),(9,'操作类型','sys_oper_type',103,'2024-07-03 01:17:32','2024-07-03 01:17:32','操作类型列表'),(10,'系统状态','sys_common_status',103,'2024-07-03 01:17:32','2024-07-03 01:17:32','登录状态列表'),(11,'授权类型','sys_grant_type',103,'2024-07-03 01:17:32','2024-07-03 01:17:32','认证授权类型'),(12,'设备类型','sys_device_type',103,'2024-07-03 01:17:32','2024-07-03 01:17:32','客户端设备类型'),(7229170480917385217,'定时任务','sys_job',1,'2024-08-14 01:02:34','2024-08-14 01:02:34','定时任务类型'),(7230795611045498880,'任务分组','sys_job_group',3,'2024-08-18 12:40:15','2024-08-18 12:40:15','定时任务分组'),(7233442155891430400,'日志记录','sys_logcache',2,'2024-08-25 19:56:41','2024-08-25 20:46:41','日志记录'),(7235877375018177536,'数据权限','sys_data_scope',4,'2024-09-01 13:13:22','2024-09-01 13:13:22','数据权限');
/*!40000 ALTER TABLE `sys_dict_type` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_job`
--

DROP TABLE IF EXISTS `sys_job`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_job` (
  `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `job_id` bigint(20) NOT NULL,
  `task_type` char(32) NOT NULL,
  `task_count` int(11) NOT NULL,
  `run_count` int(11) NOT NULL COMMENT '已执行',
  `job_name` char(32) NOT NULL,
  `job_params` char(32) DEFAULT NULL,
  `job_group` char(32) NOT NULL,
  `cron_expression` char(32) NOT NULL,
  `status` char(32) NOT NULL,
  `remark` varchar(512) NOT NULL,
  PRIMARY KEY (`job_id`),
  UNIQUE KEY `job_id` (`job_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_job`
--

LOCK TABLES `sys_job` WRITE;
/*!40000 ALTER TABLE `sys_job` DISABLE KEYS */;
INSERT INTO `sys_job` VALUES ('2024-08-16 00:34:21','2024-08-29 22:01:40',7227378358836924416,'geturl',0,1737,'请求百度','https://www.baidu.com/','default','30 * * * * ?','1','66'),('2024-08-16 00:34:21','2024-08-29 22:01:44',7227378358836924417,'geturl',0,3479,'请求QQ','https://www.qq.com/','default','0/30 * * * * ?','1','99'),('2024-09-01 12:35:38','2024-09-01 22:24:37',7235988676738454528,'invokefunction',0,13,'清理过期的Token','{\"callfun\":\"clearuserinfo\"}','default','* * 2 * * ?','1','晚上两点执行清理过期的Token');
/*!40000 ALTER TABLE `sys_job` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_job_log`
--

DROP TABLE IF EXISTS `sys_job_log`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_job_log` (
  `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `id` bigint(20) NOT NULL,
  `job_id` bigint(20) NOT NULL,
  `run_count` int(11) NOT NULL,
  `job_message` varchar(2048) DEFAULT NULL,
  `status` char(32) NOT NULL,
  `elapsed_time` datetime DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  UNIQUE KEY `id` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_job_log`
--

LOCK TABLES `sys_job_log` WRITE;
/*!40000 ALTER TABLE `sys_job_log` DISABLE KEYS */;
/*!40000 ALTER TABLE `sys_job_log` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_login_info`
--

DROP TABLE IF EXISTS `sys_login_info`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_login_info` (
  `info_id` bigint(20) NOT NULL COMMENT '访问ID',
  `uid` bigint(20) NOT NULL,
  `user_name` varchar(50) NOT NULL DEFAULT '' COMMENT '用户账号',
  `device_type` varchar(32) DEFAULT '' COMMENT '设备类型',
  `ipaddr` varchar(128) DEFAULT '' COMMENT '登录IP地址',
  `login_location` varchar(255) DEFAULT '' COMMENT '登录地点',
  `net_work` varchar(255) DEFAULT NULL,
  `browser` varchar(50) DEFAULT '' COMMENT '浏览器类型',
  `os` varchar(50) DEFAULT '' COMMENT '操作系统',
  `status` char(1) DEFAULT '0' COMMENT '登录状态（0成功 1失败）',
  `msg` varchar(255) DEFAULT '' COMMENT '提示消息',
  `login_time` datetime DEFAULT NULL COMMENT '访问时间',
  PRIMARY KEY (`info_id`),
  KEY `idx_sys_logininfor_s` (`status`),
  KEY `idx_sys_logininfor_lt` (`login_time`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='系统访问记录';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_login_info`
--

LOCK TABLES `sys_login_info` WRITE;
/*!40000 ALTER TABLE `sys_login_info` DISABLE KEYS */;
INSERT INTO `sys_login_info` VALUES (7338250336584242177,1,'admin','Other','59.41.188.149','广东省广州市','电信ADSL','Edge 137','Windows 10','0','Success','2025-06-11 01:06:58'),(7348334080074617857,1,'admin','Other','59.41.191.15','广东省广州市','电信ADSL','Other','Other','0','Success','2025-07-08 20:56:10'),(7350937685495550977,1,'admin','Other','119.130.140.63','广东省广州市','电信','Edge 138','Windows 10','0','Success','2025-07-16 01:21:58'),(7351264641348965377,1,'admin','Other','119.130.140.63','广东省广州市','电信','Edge 138','Windows 10','0','Success','2025-07-16 23:01:10'),(7395848889619616768,0,'admin','Other','61.140.94.79','广东省广州市','电信','Edge 142','Windows 10','1','Incorrect password','2025-11-16 23:43:03'),(7395848975426688001,1,'admin','Other','61.140.94.79','广东省广州市','电信','Edge 142','Windows 10','0','Success','2025-11-16 23:43:23'),(7402412371378869249,1,'admin','Other','61.140.93.246','广东省广州市','电信','Edge 142','Windows 10','0','Success','2025-12-05 02:23:58'),(7402695032978707457,1,'admin','Other','61.140.93.246','广东省广州市','电信','Edge 142','Windows 10','0','Success','2025-12-05 21:07:11'),(7402709166432621569,1,'admin','Other','61.140.93.246','广东省广州市','电信','Other','Other','0','Success','2025-12-05 22:03:21'),(7402714375351342080,0,'test3','Other','61.140.93.246','广东省广州市','电信','Chrome 142','Windows 10','1','Incorrect password','2025-12-05 22:24:03'),(7402714547561075712,0,'test3','Other','61.140.93.246','广东省广州市','电信','Chrome 142','Windows 10','1','Incorrect password','2025-12-05 22:24:44'),(7402714693057287169,7222295451336314881,'test3','Other','61.140.93.246','广东省广州市','电信','Chrome 142','Windows 10','0','Success','2025-12-05 22:25:17'),(7402715852585210881,7222295451336314881,'test3','Other','61.140.93.246','广东省广州市','电信','Chrome 142','Windows 10','0','Success','2025-12-05 22:29:54'),(7402728157159003137,7402727913407026176,'test6','Other','61.140.93.246','广东省广州市','电信','Chrome 142','Windows 10','0','Success','2025-12-05 23:18:48'),(7402749118889497600,1,'admin','Other','61.140.93.246','广东省广州市','电信','Other','Other','0','Success','2025-12-06 00:42:06'),(7402749256399754241,7222295451336314881,'test3','Other','61.140.93.246','广东省广州市','电信','Other','Other','0','Success','2025-12-06 00:42:39'),(7403508110842565633,1,'admin','Other','219.136.129.196','广东省广州市','电信ADSL','Chrome 142','Windows 10','0','Success','2025-12-08 02:58:03'),(7403508317604975616,7402727913407026176,'test6','Other','219.136.129.196','广东省广州市','电信ADSL','Chrome 142','Windows 10','0','Success','2025-12-08 02:58:52');
/*!40000 ALTER TABLE `sys_login_info` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_menu`
--

DROP TABLE IF EXISTS `sys_menu`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_menu` (
  `id` bigint(20) NOT NULL COMMENT '菜单ID',
  `name` varchar(32) DEFAULT NULL COMMENT '菜单名称',
  `title` varchar(32) NOT NULL,
  `i18nkey` varchar(32) DEFAULT NULL,
  `pid` bigint(20) NOT NULL DEFAULT '0' COMMENT '父菜单ID',
  `order` int(4) NOT NULL DEFAULT '0' COMMENT '显示顺序',
  `path` varchar(200) DEFAULT '' COMMENT '路由地址',
  `component` varchar(255) DEFAULT NULL COMMENT '组件路径',
  `redirect` varchar(100) DEFAULT NULL COMMENT '重定向',
  `href` varchar(32) DEFAULT '' COMMENT '是否为外链',
  `no_cache` char(1) NOT NULL DEFAULT '0' COMMENT '是否缓存（0缓存 1不缓存）',
  `menu_type` char(1) NOT NULL DEFAULT '' COMMENT '菜单类型（M目录 C菜单 F按钮）',
  `hidden` char(1) NOT NULL DEFAULT '0' COMMENT '显示状态（0显示 1隐藏）',
  `active_menu` char(1) NOT NULL DEFAULT '0' COMMENT '显示高亮的路由路径',
  `always_show` char(1) NOT NULL DEFAULT '0',
  `breadcrumb` char(1) NOT NULL DEFAULT '1' COMMENT '如果设置为false，则不会在breadcrumb面包屑中显示(默认 true)',
  `affix` char(1) NOT NULL DEFAULT '0' COMMENT '如果设置为true，则会一直固定在tag项中(默认 false)',
  `no_tags_view` char(1) NOT NULL DEFAULT '0' COMMENT '如果设置为true，则不会出现在tag中(默认 false)',
  `can_to` char(1) NOT NULL DEFAULT '0' COMMENT ' 设置为true即使hidden为true，也依然可以进行路由跳转(默认 false)',
  `status` char(1) NOT NULL DEFAULT '1' COMMENT '菜单状态（0正常 1停用）',
  `perms` varchar(100) DEFAULT NULL COMMENT '权限标识',
  `icon` varchar(100) DEFAULT '#' COMMENT '菜单图标',
  `created_at` datetime DEFAULT NULL COMMENT '创建时间',
  `updated_at` datetime DEFAULT NULL COMMENT '更新时间',
  `remark` varchar(500) DEFAULT '' COMMENT '备注',
  `deleted_at` datetime DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='菜单权限表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_menu`
--

LOCK TABLES `sys_menu` WRITE;
/*!40000 ALTER TABLE `sys_menu` DISABLE KEYS */;
INSERT INTO `sys_menu` VALUES (1,'Dashboard','首页',NULL,0,0,'/dashboard','#','/dashboard/analysis','','0','M','0','0','0','1','0','1','1','0',NULL,'vi-ant-design:dashboard-filled',NULL,NULL,'',NULL),(2,'Analysis','分析页',NULL,1,1,'analysis','views/Dashboard/Analysis',NULL,'','0','C','0','0','0','1','0','1','1','0',NULL,'vi-ant-design:dashboard-filled',NULL,'2024-08-23 22:51:09',NULL,NULL),(3,'Workplace','工作台',NULL,1,2,'workplace','views/Dashboard/Workplace',NULL,'','0','C','0','0','0','1','0','1','1','0',NULL,'vi-ant-design:codepen-outlined',NULL,'2024-08-23 22:50:07',NULL,NULL),(4,'Authorization','系统管理',NULL,0,2,'/authorization','#',NULL,'','0','M','0','0','0','1','0','1','1','0',NULL,'vi-ant-design:dashboard-filled',NULL,NULL,'',NULL),(5,'Department','部门管理',NULL,4,5,'department','views/Authorization/Department/Department',NULL,'','0','C','0','0','0','1','0','1','1','0',NULL,'vi-ant-design:dashboard-filled',NULL,'2024-08-11 05:31:40',NULL,NULL),(6,'User','用户管理',NULL,4,2,'user','views/Authorization/User/User',NULL,'','0','C','0','0','0','1','0','1','1','0',NULL,'vi-ant-design:dashboard-filled',NULL,NULL,'',NULL),(7,'Menu','菜单管理',NULL,4,2,'menu','views/Authorization/Menu/Menu',NULL,'','0','C','0','0','0','1','0','0','1','0',NULL,'vi-ant-design:dashboard-filled',NULL,'2024-08-08 00:50:11',NULL,NULL),(8,'Role','角色管理',NULL,4,2,'role','views/Authorization/Role/Role',NULL,'','0','C','0','0','0','1','0','1','1','0',NULL,'vi-ant-design:dashboard-filled',NULL,NULL,'',NULL),(7224471859710005249,'Monitor','系统监控',NULL,0,5,'/motion','#',NULL,NULL,'0','M','0','0','0','1','0','1','1','0',NULL,'vi-tdesign:archway',NULL,'2024-10-25 00:42:23',NULL,NULL),(7225437966700777473,'SystemLog','操作日志',NULL,7224471859710005249,4,'systemlog','views/Monitor/SystemLog/SystemLog',NULL,NULL,'0','C','0','0','0','1','0','0','0','0',NULL,'vi-tdesign:angry',NULL,'2024-08-28 01:49:53',NULL,NULL),(7225439414641627136,'LoginInfo','登陆日志',NULL,7224471859710005249,2,'logininfo','views/Monitor/LoginInfo/LoginInfo',NULL,NULL,'0','C','0','0','0','1','0','0','0','0',NULL,'vi-tdesign:app',NULL,'2024-08-25 00:53:36',NULL,NULL),(7226994818660044800,'UserOnline','在线用户',NULL,7224471859710005249,0,'useronline','views/Monitor/UserOnline/UserOnline',NULL,'','0','C','0','0','0','1','0','1','1','0',NULL,'vi-ep:bell-filled','2024-08-08 00:57:16','2024-10-24 01:28:11',NULL,NULL),(7227715297679118337,'Job','定时任务',NULL,7224471859710005249,0,'job','views/Monitor/Job/Job',NULL,'','0','C','0','0','0','0','0','0','0','0',NULL,'vi-ep:apple','2024-08-10 00:40:11','2024-08-10 00:40:11',NULL,NULL),(7228150987080470529,'DictData','字典数据',NULL,4,100,'dictdata','views/Authorization/DictData/DictData',NULL,'','0','C','1','0','0','0','0','1','0','0',NULL,'vi-ant-design:aliwangwang-outlined','2024-08-11 05:31:28','2024-10-25 23:18:20',NULL,NULL),(7228186533035118593,'DictType','字典管理',NULL,4,7,'dicttype','views/Authorization/DictType/DictType',NULL,'','0','C','0','0','0','0','0','0','0','0',NULL,'vi-ant-design:alert-filled','2024-08-11 07:52:43','2024-10-25 23:18:11',NULL,NULL),(7228454585970593792,'ServerInfo','服务监控',NULL,7224471859710005249,4,'serverinfo','views/Monitor/ServerInfo/ServerInfo',NULL,'','1','C','0','0','0','0','0','0','0','0',NULL,'vi-ant-design:fund-projection-screen-outlined','2024-08-12 01:37:51','2024-08-25 00:54:16',NULL,NULL),(7230621134038896640,'JobLog','定时日志',NULL,7224471859710005249,0,'joblog','views/Monitor/JobLog/JobLog',NULL,'','1','C','1','0','0','0','0','0','0','0',NULL,'vi-ep:baseball','2024-08-18 01:06:57','2024-09-01 00:09:39',NULL,NULL),(7230940389695426560,'ApiPermission','权限查询',NULL,4,7,'apipermission','views/Authorization/ApiPermission/ApiPermission',NULL,'','0','C','0','0','0','0','0','0','0','0',NULL,'vi-ep:burger','2024-08-18 22:15:33','2024-08-24 01:22:22',NULL,NULL),(7231673632416502784,'RoleApi','角色API',NULL,4,1,'roleapi','views/Authorization/RoleApi/RoleApi',NULL,'','0','C','1','0','0','0','0','0','0','0',NULL,'vi-ep:basketball','2024-08-20 22:49:12','2024-10-24 01:32:15',NULL,NULL),(7234621748841190400,'Cache','缓存管理',NULL,7224471859710005249,1,'cache','views/Monitor/Cache/Cache',NULL,'','0','C','0','0','0','0','0','0','0','0',NULL,'vi-ep:bicycle','2024-08-29 02:03:58','2024-08-29 02:03:58',NULL,NULL),(7402695499167208448,NULL,'测试',NULL,0,15,'/test','#',NULL,'','0','M','0','0','0','0','0','0','0','0',NULL,'vi-tdesign:application','2025-12-05 21:09:02','2025-12-05 21:09:02',NULL,NULL),(7402702074216879104,'TestDataScope','测试数据',NULL,7402695499167208448,1,'test/TestDataScope','views/Test/TestDataScope/TestDataScope',NULL,'','0','C','0','0','0','0','0','0','0','0',NULL,'vi-tdesign:animation-1','2025-12-05 21:35:10','2025-12-05 21:35:51',NULL,NULL);
/*!40000 ALTER TABLE `sys_menu` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_notice`
--

DROP TABLE IF EXISTS `sys_notice`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_notice` (
  `notice_id` bigint(20) NOT NULL COMMENT '公告ID',
  `notice_title` varchar(50) NOT NULL COMMENT '公告标题',
  `notice_type` char(1) NOT NULL COMMENT '公告类型（1通知 2公告）',
  `notice_content` longblob COMMENT '公告内容',
  `status` char(1) DEFAULT '0' COMMENT '公告状态（0正常 1关闭）',
  `create_dept` bigint(20) DEFAULT NULL COMMENT '创建部门',
  `create_by` bigint(20) DEFAULT NULL COMMENT '创建者',
  `created_at` datetime DEFAULT NULL COMMENT '创建时间',
  `update_by` bigint(20) DEFAULT NULL COMMENT '更新者',
  `updated_at` datetime DEFAULT NULL COMMENT '更新时间',
  `remark` varchar(255) DEFAULT NULL COMMENT '备注',
  PRIMARY KEY (`notice_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='通知公告表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_notice`
--

LOCK TABLES `sys_notice` WRITE;
/*!40000 ALTER TABLE `sys_notice` DISABLE KEYS */;
INSERT INTO `sys_notice` VALUES (1,'温馨提醒：2018-07-01 新版本发布啦','2',_binary '新版本内容','0',103,1,'2024-07-03 01:17:33',NULL,NULL,'管理员'),(2,'维护通知：2018-07-01 系统凌晨维护','1',_binary '维护内容','0',103,1,'2024-07-03 01:17:33',NULL,NULL,'管理员');
/*!40000 ALTER TABLE `sys_notice` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_oper_log`
--

DROP TABLE IF EXISTS `sys_oper_log`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_oper_log` (
  `oper_id` bigint(20) NOT NULL COMMENT '日志主键',
  `api_name` varchar(50) DEFAULT '',
  `method` varchar(100) DEFAULT '' COMMENT '方法名称',
  `request_method` varchar(10) DEFAULT '' COMMENT '请求方式',
  `oper_name` varchar(50) DEFAULT '' COMMENT '操作人员',
  `oper_url` varchar(255) DEFAULT '' COMMENT '请求URL',
  `oper_ip` varchar(128) DEFAULT '' COMMENT '主机地址',
  `oper_location` varchar(255) DEFAULT '' COMMENT '操作地点',
  `oper_param` varchar(2048) DEFAULT '' COMMENT '请求参数',
  `json_result` varchar(2048) DEFAULT '' COMMENT '返回参数',
  `status` char(1) DEFAULT '0' COMMENT '操作状态（0正常 1异常）',
  `error_msg` varchar(2000) DEFAULT '' COMMENT '错误消息',
  `oper_time` datetime DEFAULT NULL COMMENT '操作时间',
  `cost_time` bigint(20) DEFAULT '0' COMMENT '消耗时间',
  PRIMARY KEY (`oper_id`),
  KEY `idx_sys_oper_log_s` (`status`),
  KEY `idx_sys_oper_log_ot` (`oper_time`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='操作日志记录';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_oper_log`
--

LOCK TABLES `sys_oper_log` WRITE;
/*!40000 ALTER TABLE `sys_oper_log` DISABLE KEYS */;
/*!40000 ALTER TABLE `sys_oper_log` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_post`
--

DROP TABLE IF EXISTS `sys_post`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_post` (
  `post_id` bigint(20) NOT NULL COMMENT '岗位ID',
  `dept_id` bigint(20) NOT NULL COMMENT '部门id',
  `post_code` varchar(64) NOT NULL COMMENT '岗位编码',
  `post_category` varchar(100) DEFAULT NULL COMMENT '岗位类别编码',
  `post_name` varchar(50) NOT NULL COMMENT '岗位名称',
  `post_sort` int(4) NOT NULL COMMENT '显示顺序',
  `status` char(1) NOT NULL COMMENT '状态（0正常 1停用）',
  `create_dept` bigint(20) DEFAULT NULL COMMENT '创建部门',
  `created_at` datetime DEFAULT NULL COMMENT '创建时间',
  `updated_at` datetime DEFAULT NULL COMMENT '更新时间',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  PRIMARY KEY (`post_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='岗位信息表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_post`
--

LOCK TABLES `sys_post` WRITE;
/*!40000 ALTER TABLE `sys_post` DISABLE KEYS */;
INSERT INTO `sys_post` VALUES (1,103,'ceo',NULL,'董事长',1,'0',103,'2024-07-03 01:17:32',NULL,''),(2,100,'se',NULL,'项目经理',2,'0',103,'2024-07-03 01:17:32',NULL,''),(3,100,'hr',NULL,'人力资源',3,'0',103,'2024-07-03 01:17:32',NULL,''),(4,100,'user',NULL,'普通员工',4,'0',103,'2024-07-03 01:17:32',NULL,'');
/*!40000 ALTER TABLE `sys_post` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_role`
--

DROP TABLE IF EXISTS `sys_role`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_role` (
  `role_id` bigint(20) NOT NULL COMMENT '角色ID',
  `role_name` varchar(30) NOT NULL COMMENT '角色名称',
  `role_key` varchar(100) NOT NULL COMMENT '角色权限字符串',
  `order` int(4) NOT NULL COMMENT '显示顺序',
  `data_scope` char(1) NOT NULL DEFAULT '1' COMMENT '数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）',
  `status` char(1) NOT NULL COMMENT '角色状态（0正常 1停用）',
  `create_dept` bigint(20) DEFAULT NULL COMMENT '创建部门',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_at` datetime DEFAULT NULL COMMENT '创建时间',
  `updated_at` datetime DEFAULT NULL COMMENT '更新时间',
  `deleted_at` datetime DEFAULT NULL,
  PRIMARY KEY (`role_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='角色信息表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_role`
--

LOCK TABLES `sys_role` WRITE;
/*!40000 ALTER TABLE `sys_role` DISABLE KEYS */;
INSERT INTO `sys_role` VALUES (1,'管理员','admin',2,'1','0',103,'超级管理员','2024-07-03 01:17:32',NULL,NULL),(3,'本部门及以下','test1',5,'4','0',103,'部门数据以及部门以下的数据','2024-07-03 01:17:32',NULL,NULL),(4,'本人的数据权限','self',6,'5','0',103,'本人数据','2024-07-03 01:17:32',NULL,NULL),(7222985345490620417,'自定义数据权限','self',7,'2','0',NULL,'自定义数据权限',NULL,NULL,NULL),(7403403929607640064,'本部门权限','Dept',5,'3','0',NULL,'本部门数据',NULL,NULL,NULL);
/*!40000 ALTER TABLE `sys_role` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_role_api`
--

DROP TABLE IF EXISTS `sys_role_api`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_role_api` (
  `id` bigint(20) NOT NULL,
  `role_id` bigint(20) NOT NULL,
  `api_id` bigint(20) NOT NULL,
  `api` char(255) NOT NULL,
  `method` char(10) NOT NULL,
  `apiname` char(32) NOT NULL,
  `sort` int(11) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_role_api`
--

LOCK TABLES `sys_role_api` WRITE;
/*!40000 ALTER TABLE `sys_role_api` DISABLE KEYS */;
INSERT INTO `sys_role_api` VALUES (7402715253093340160,7222985345490620417,7232752187397280768,'/sys/job/add','Post','添加定时任务',1),(7402715253202392064,7222985345490620417,7232752187535692800,'/sys/job/validate_cron','Post','验证cron表达式',2),(7402715253261112320,7222985345490620417,7232752187606995968,'/sys/job/list','Get','获取定时任务列表',3),(7402715253307249664,7222985345490620417,7232752187674104832,'/sys/job/edit','Put','编辑定时任务',4),(7402715253361775616,7222985345490620417,7232752187741213696,'/sys/job/del','Delete','删除定时任务',5),(7402715253412107264,7222985345490620417,7232752187812516864,'/sys/roleapi/add_many_role_api_transfer','Get','添加多个角色api',6),(7402715253462438912,7222985345490620417,7232752187879625728,'/sys/roleapi/del','Delete','删除角色api',7),(7402715253512770560,7222985345490620417,7232752187946734592,'/sys/roleapi/role_api_transfer_list','Get','获取角色api所有的选择列表',8),(7402715253558907904,7222985345490620417,7232752188013843456,'/sys/roleapi/list','Get','获取角色api列表',9),(7402715253609239552,7222985345490620417,7232752188085146624,'/sys/roleapi/edit','Put','编辑角色api',10),(7402715253655376896,7222985345490620417,7232752188156449792,'/sys/roleapi/role_permission_list','Get','根据角色id获取api权限列表',11),(7402715253705708544,7222985345490620417,7232752188219364352,'/sys/roleapi/add','Post','添加角色api',12),(7402715253751845888,7222985345490620417,7235633673968456704,'/sys/role/get_role_depts','Get','获取角色部门',13),(7402715253802177536,7222985345490620417,7232752188282278912,'/sys/dicttype/list','Get','获取字典类型列表',14),(7402715253856703488,7222985345490620417,7252729164858299392,'/sys/role/user_role_name_list','Get','获取用户拥有的角色名称列表',15),(7402715253898646528,7222985345490620417,7232752188349387776,'/sys/dicttype/add','Post','添加字典类型',16),(7402715253957366784,7222985345490620417,7232752188412302336,'/sys/dicttype/del','Delete','删除字典类型',17),(7402715254011892736,7222985345490620417,7232752188479411200,'/sys/dicttype/edit','Put','编辑字典类型',18),(7402715254053835776,7222985345490620417,7232752188546520064,'/sys/menu/edit','Put','编辑菜单',19),(7402715254112556032,7222985345490620417,7234616780964926464,'/sys/cache/clear','Post','清空缓存',20),(7402715254162887680,7222985345490620417,7232752188617823232,'/sys/menu/list','Get','菜单列表',21),(7402715254234190848,7222985345490620417,7234616781027841024,'/sys/cache/list','Get','获取缓存列表',22),(7402715254288716800,7222985345490620417,7232752188697515008,'/sys/menu/add','Post','添加菜单',23),(7402715254339048448,7222985345490620417,7232752188764623872,'/sys/menu/del','Delete','删除菜单',24),(7402715254376797184,7222985345490620417,7232752188835927040,'/sys/menu/all_router','Get','全部路由',25),(7402715254422934528,7222985345490620417,7232752188898841600,'/sys/menu/tree','Get','获取菜单树',26),(7402715254469071872,7222985345490620417,7232752188965950464,'/sys/serverinfo/server_update','Get','更新服务器信息',27),(7402715254515209216,7222985345490620417,7232752189028865024,'/sys/jobinfo/add','Post','添加定时任务日志',28),(7402715254565540864,7222985345490620417,7232794958103483392,'/sys/dashboard/analysis/weeklyUserActivity','Get','获取仪表盘数据',29),(7402715254620066816,7222985345490620417,7232752189095973888,'/sys/jobinfo/list','Get','获取定时任务日志列表',30),(7402715254674592768,7222985345490620417,7232794958170592256,'/sys/dashboard/analysis/total','Get','获取仪表盘数据',31),(7402715254729118720,7222985345490620417,7232781502121939968,'/sys/user/delusers','Delete','删除用户',32),(7402715254771061760,7222985345490620417,7232752189158888448,'/sys/dictdata/get_by_type','Get','根据类型获取字典数据',33),(7402715254821393408,7222985345490620417,7232794958229312512,'/sys/dashboard/analysis/monthlySales','Get','获取仪表盘数据',34),(7402715254863336448,7222985345490620417,7232794958279644160,'/sys/dashboard/analysis/userAccessSource','Get','获取仪表盘数据',35),(7402715254909473792,7222985345490620417,7232752189221803008,'/sys/dictdata/list','Get','获取字典数据列表',36),(7402715254959805440,7222985345490620417,7232752189280523264,'/sys/dictdata/del','Delete','删除字典数据',37),(7402715255005942784,7222985345490620417,7232794958317392896,'/sys/dashboard/workplace/dynamic','Get','获取仪表盘数据',38),(7402715255056274432,7222985345490620417,7232794958363530240,'/sys/dashboard/workplace/total','Get','获取仪表盘数据',39),(7402715255102411776,7222985345490620417,7232752189347632128,'/sys/dictdata/edit','Put','编辑字典数据',40),(7402715255148549120,7222985345490620417,7232752189410546688,'/sys/dictdata/add','Post','添加字典数据',41),(7402715255207269376,7222985345490620417,7232794958401278976,'/sys/dashboard/workplace/radar','Get','获取仪表盘数据',42),(7402715255261795328,7222985345490620417,7232794958447416320,'/sys/dashboard/workplace/project','Get','获取仪表盘数据',43),(7402715255307932672,7222985345490620417,7232752189473461248,'/sys/dashboard/add','Post','添加仪表盘',44),(7402715255354070016,7222985345490620417,7232794958493553664,'/sys/dashboard/workplace/team','Get','获取仪表盘数据',45),(7402715255396013056,7222985345490620417,7232752189557347328,'/sys/dashboard/list','Get','获取仪表盘列表',46),(7402715255450539008,7222985345490620417,7232752189628650496,'/sys/role/list','Get','获取角色列表',47),(7402715255496676352,7222985345490620417,7232752189691565056,'/sys/role/get_role_menus','Get','获取角色菜单',48),(7402715255542813696,7222985345490620417,7232752189758673920,'/sys/role/menu','Get','获取角色菜单',49),(7402715255597339648,7222985345490620417,7236784152022782976,'/sys/user/fresh_token','Put','刷新token',50),(7402715255643476992,7222985345490620417,7232752189821588480,'/sys/role/edit','Put','编辑角色',51),(7402715255685420032,7222985345490620417,7232752189888697344,'/sys/role/tree','Get','获取角色树',52),(7402715255727363072,7222985345490620417,7232752189951611904,'/sys/role/add','Post','添加角色',53),(7402715255773500416,7222985345490620417,7236784153973134336,'/sys/user/update_avatar','Put','更新头像',54),(7402715255819637760,7222985345490620417,7232752190018720768,'/sys/role/del','Delete','删除角色',55),(7402715255861580800,7222985345490620417,7232752190081635328,'/sys/apipermission/list','Get','获取api权限列表',56),(7402715255903523840,7222985345490620417,7232752190144549888,'/sys/user/reset_password','Put','重置密码',57),(7402715255949661184,7222985345490620417,7232752190207464448,'/sys/user/userinfo','Get','获取用户信息',58),(7402715255991604224,7222985345490620417,7232752190270379008,'/sys/user/depts_roles','Get','获取用户拥有的用户部门和用户角色',59),(7402715256029352960,7222985345490620417,7232752190333293568,'/sys/user/del','Delete','删除用户',60),(7402715256075490304,7222985345490620417,7252738573697192960,'/sys/dept/user_dept_name_list','Get','获取用户拥有的部门名称列表',61),(7402715256113239040,7222985345490620417,7232752190396208128,'/sys/user/list','Get','获取用户列表',62),(7402715256159376384,7222985345490620417,7232752190459122688,'/sys/user/add','Post','添加用户',63),(7402715256205513728,7222985345490620417,7232752190526231552,'/sys/user/change_password','Put','修改密码',64),(7402715256289399808,7222985345490620417,7234570580194661376,'/sys/useronline/del','Delete','退出在线用户',65),(7402715256339731456,7222985345490620417,7232752190589146112,'/sys/user/edit','Put','编辑用户',66),(7402715256385868800,7222985345490620417,7232752190656254976,'/sys/dept/edit','Put','编辑部门',67),(7402715256427811840,7222985345490620417,7232752190727558144,'/sys/dept/add','Post','添加部门',68),(7402715256473949184,7222985345490620417,7232752190794667008,'/sys/dept/list','Get','获取部门列表',69),(7402715256515892224,7222985345490620417,7234253747373642752,'/sys/operationlog/list','Get','获取操作日志列表',70),(7402715256578806784,7222985345490620417,7232752190861775872,'/sys/dept/del','Delete','删除部门',71),(7402715256629138432,7222985345490620417,7232752190933079040,'/sys/dept/dept_tree','Get','获取部门树',72),(7402715256687858688,7222985345490620417,7232752190995993600,'/sys/useronline/list','Get','获取在线用户列表',73),(7402715256733996032,7222985345490620417,7232752191063102464,'/sys/logininfo/list','Get','获取登录日志列表',74),(7402715256784327680,7222985345490620417,7253067149336286208,'/sys/user/update_role_or_dept','Post','更新用户拥有的用户部门和用户角色',75),(7402715256834659328,7222985345490620417,7233868069737501696,'/sys/apipermission/edit','Put','编辑api权限',76),(7402715256884990976,7222985345490620417,7241490807281062912,'/sys/user/login_out','Get','用户退出',77),(7402715256931128320,7222985345490620417,7240020747752477696,'/sys/user/refersh_token','Put','刷新token',78),(7402715256968877056,7222985345490620417,7235678367742071808,'/sys/job/hand_execute_job','Post','执行定时任务',79),(7402715257023403008,7222985345490620417,7385368752226735104,'/test/test_api/list','Get','获取列表',80),(7402715257073734656,7222985345490620417,7385368752235123712,'/test/test_api/del','Delete','删除TestApi',81),(7402715257119872000,7222985345490620417,7401293722618336256,'/test/test_api/db_read_write_test','Put','db_read_write_test',82),(7402715257174397952,7222985345490620417,7348333842530210816,'/wechat/wxaccounts/del','Delete','删除WxAccounts',83),(7402715257212146688,7222985345490620417,7385368752239318016,'/test/test_api/add','Post','添加TestApi',84),(7402715257262478336,7222985345490620417,7401293722630919168,'/test/test_api/db_name_test','Put','db_name_test',85),(7402715257308615680,7222985345490620417,7348333842559570944,'/wechat/wxaccounts/edit','Put','编辑WxAccounts',86),(7402715257354753024,7222985345490620417,7348333842567959552,'/wechat/wxaccounts/list','Get','获取WxAccounts列表',87),(7402715257400890368,7222985345490620417,7385368752243512320,'/test/test_api/edit','Put','编辑TestApi',88),(7402715257451222016,7222985345490620417,7348333842572153856,'/wechat/wxaccounts/add','Post','添加WxAccounts',89),(7402715257497359360,7222985345490620417,7348333842584736768,'/wechat/wxautoreplies/add','Post','添加WxAutoReplies',90),(7402715257547691008,7222985345490620417,7401293722647696384,'/test/test_api/db_index_test','Put','db_index_test',91),(7402715257589634048,7222985345490620417,7348333842609902592,'/wechat/wxautoreplies/del','Delete','删除WxAutoReplies',92),(7402715257644160000,7222985345490620417,7348333842635068416,'/wechat/wxautoreplies/list','Get','获取WxAutoReplies列表',93),(7402715257698685952,7222985345490620417,7401293722651890688,'/test/test_api/db_auto_test','Put','db_auto_test',94),(7402715257749017600,7222985345490620417,7348333843008361472,'/wechat/wxautoreplies/edit','Put','编辑WxAutoReplies',95),(7402715257790960640,7222985345490620417,7402694868939478016,'/test/test_data_scope/del','Delete','Delete',96),(7402715257832903680,7222985345490620417,7402694868947866624,'/test/test_data_scope/list','Get','list',97),(7402715257883235328,7222985345490620417,7348333843016750080,'/wechat/wxmenus/list','Get','获取WxMenus列表',98),(7402715257929372672,7222985345490620417,7348333843071276032,'/wechat/wxmenus/del','Delete','删除WxMenus',99),(7402715257975510016,7222985345490620417,7402694868947866625,'/test/test_data_scope/add','Post','add',100),(7402715258034230272,7222985345490620417,7402694868952060928,'/test/test_data_scope/edit','Put','edit',101),(7402715258088756224,7222985345490620417,7348333843075470336,'/wechat/wxmenus/pull_menu','Get','创建自定义菜单',102),(7402715258134893568,7222985345490620417,7348333843092247552,'/wechat/wxmenus/add','Post','添加WxMenus',103),(7402715258185225216,7222985345490620417,7348333843100636160,'/wechat/wxmenus/edit','Put','编辑WxMenus',104),(7402715258231362560,7222985345490620417,7348333843104830464,'/wechat/wxmessages/list','Get','获取WxMessages列表',105),(7402715258281694208,7222985345490620417,7348333843125801984,'/wechat/wxmessages/edit','Put','编辑WxMessages',106),(7402715258332025856,7222985345490620417,7348333843138384896,'/wechat/wxmessages/del','Delete','删除WxMessages',107),(7402715258403329024,7222985345490620417,7348333843150967808,'/wechat/wxmessages/add','Post','添加WxMessages',108),(7402715258453660672,7222985345490620417,7348333843159356416,'/wechat/wxusers/add','Post','添加WxUsers',109),(7402715258495603712,7222985345490620417,7348333843167745024,'/wechat/wxusers/edit','Put','编辑WxUsers',110),(7402715258545935360,7222985345490620417,7348333843180327936,'/wechat/wxusers/del','Delete','删除WxUsers',111),(7402715258587878400,7222985345490620417,7348333843184522240,'/wechat/wxusers/list','Get','获取WxUsers列表',112),(7402715541250413568,4,7232752187397280768,'/sys/job/add','Post','添加定时任务',1),(7402715541346882560,4,7232752187535692800,'/sys/job/validate_cron','Post','验证cron表达式',2),(7402715541397214208,4,7232752187606995968,'/sys/job/list','Get','获取定时任务列表',3),(7402715541430768640,4,7232752187674104832,'/sys/job/edit','Put','编辑定时任务',4),(7402715541489488896,4,7232752187741213696,'/sys/job/del','Delete','删除定时任务',5),(7402715541539820544,4,7232752187812516864,'/sys/roleapi/add_many_role_api_transfer','Get','添加多个角色api',6),(7402715541585957888,4,7232752187879625728,'/sys/roleapi/del','Delete','删除角色api',7),(7402715541636289536,4,7232752187946734592,'/sys/roleapi/role_api_transfer_list','Get','获取角色api所有的选择列表',8),(7402715541690815488,4,7232752188013843456,'/sys/roleapi/list','Get','获取角色api列表',9),(7402715541736952832,4,7232752188085146624,'/sys/roleapi/edit','Put','编辑角色api',10),(7402715541787284480,4,7232752188156449792,'/sys/roleapi/role_permission_list','Get','根据角色id获取api权限列表',11),(7402715541846004736,4,7232752188219364352,'/sys/roleapi/add','Post','添加角色api',12),(7402715541896336384,4,7235633673968456704,'/sys/role/get_role_depts','Get','获取角色部门',13),(7402715541946668032,4,7232752188282278912,'/sys/dicttype/list','Get','获取字典类型列表',14),(7402715541992805376,4,7252729164858299392,'/sys/role/user_role_name_list','Get','获取用户拥有的角色名称列表',15),(7402715542047331328,4,7232752188349387776,'/sys/dicttype/add','Post','添加字典类型',16),(7402715542097662976,4,7232752188412302336,'/sys/dicttype/del','Delete','删除字典类型',17),(7402715542152188928,4,7232752188479411200,'/sys/dicttype/edit','Put','编辑字典类型',18),(7402715542210909184,4,7232752188546520064,'/sys/menu/edit','Put','编辑菜单',19),(7402715542261240832,4,7234616780964926464,'/sys/cache/clear','Post','清空缓存',20),(7402715542307378176,4,7232752188617823232,'/sys/menu/list','Get','菜单列表',21),(7402715542361904128,4,7234616781027841024,'/sys/cache/list','Get','获取缓存列表',22),(7402715542420624384,4,7232752188697515008,'/sys/menu/add','Post','添加菜单',23),(7402715542466761728,4,7232752188764623872,'/sys/menu/del','Delete','删除菜单',24),(7402715542508704768,4,7232752188835927040,'/sys/menu/all_router','Get','全部路由',25),(7402715542554842112,4,7232752188898841600,'/sys/menu/tree','Get','获取菜单树',26),(7402715542605173760,4,7232752188965950464,'/sys/serverinfo/server_update','Get','更新服务器信息',27),(7402715542663894016,4,7232752189028865024,'/sys/jobinfo/add','Post','添加定时任务日志',28),(7402715542697448448,4,7232794958103483392,'/sys/dashboard/analysis/weeklyUserActivity','Get','获取仪表盘数据',29),(7402715542743585792,4,7232752189095973888,'/sys/jobinfo/list','Get','获取定时任务日志列表',30),(7402715542781334528,4,7232794958170592256,'/sys/dashboard/analysis/total','Get','获取仪表盘数据',31),(7402715542827471872,4,7232781502121939968,'/sys/user/delusers','Delete','删除用户',32),(7402715542894580736,4,7232752189158888448,'/sys/dictdata/get_by_type','Get','根据类型获取字典数据',33),(7402715542940718080,4,7232794958229312512,'/sys/dashboard/analysis/monthlySales','Get','获取仪表盘数据',34),(7402715542991049728,4,7232794958279644160,'/sys/dashboard/analysis/userAccessSource','Get','获取仪表盘数据',35),(7402715543041381376,4,7232752189221803008,'/sys/dictdata/list','Get','获取字典数据列表',36),(7402715543100101632,4,7232752189280523264,'/sys/dictdata/del','Delete','删除字典数据',37),(7402715543150433280,4,7232794958317392896,'/sys/dashboard/workplace/dynamic','Get','获取仪表盘数据',38),(7402715543221736448,4,7232794958363530240,'/sys/dashboard/workplace/total','Get','获取仪表盘数据',39),(7402715543280456704,4,7232752189347632128,'/sys/dictdata/edit','Put','编辑字典数据',40),(7402715543330788352,4,7232752189410546688,'/sys/dictdata/add','Post','添加字典数据',41),(7402715543389508608,4,7232794958401278976,'/sys/dashboard/workplace/radar','Get','获取仪表盘数据',42),(7402715543444034560,4,7232794958447416320,'/sys/dashboard/workplace/project','Get','获取仪表盘数据',43),(7402715543485977600,4,7232752189473461248,'/sys/dashboard/add','Post','添加仪表盘',44),(7402715543548892160,4,7232794958493553664,'/sys/dashboard/workplace/team','Get','获取仪表盘数据',45),(7402715543603418112,4,7232752189557347328,'/sys/dashboard/list','Get','获取仪表盘列表',46),(7402715543666332672,4,7232752189628650496,'/sys/role/list','Get','获取角色列表',47),(7402715543720858624,4,7232752189691565056,'/sys/role/get_role_menus','Get','获取角色菜单',48),(7402715543787967488,4,7232752189758673920,'/sys/role/menu','Get','获取角色菜单',49),(7402715543842493440,4,7236784152022782976,'/sys/user/fresh_token','Put','刷新token',50),(7402715543897019392,4,7232752189821588480,'/sys/role/edit','Put','编辑角色',51),(7402715543955739648,4,7232752189888697344,'/sys/role/tree','Get','获取角色树',52),(7402715544006071296,4,7232752189951611904,'/sys/role/add','Post','添加角色',53),(7402715544052208640,4,7236784153973134336,'/sys/user/update_avatar','Put','更新头像',54),(7402715544106734592,4,7232752190018720768,'/sys/role/del','Delete','删除角色',55),(7402715544157066240,4,7232752190081635328,'/sys/apipermission/list','Get','获取api权限列表',56),(7402715544215786496,4,7232752190144549888,'/sys/user/reset_password','Put','重置密码',57),(7402715544261923840,4,7232752190207464448,'/sys/user/userinfo','Get','获取用户信息',58),(7402715544303866880,4,7232752190270379008,'/sys/user/depts_roles','Get','获取用户拥有的用户部门和用户角色',59),(7402715544350004224,4,7232752190333293568,'/sys/user/del','Delete','删除用户',60),(7402715544387752960,4,7252738573697192960,'/sys/dept/user_dept_name_list','Get','获取用户拥有的部门名称列表',61),(7402715544433890304,4,7232752190396208128,'/sys/user/list','Get','获取用户列表',62),(7402715544480027648,4,7232752190459122688,'/sys/user/add','Post','添加用户',63),(7402715544534553600,4,7232752190526231552,'/sys/user/change_password','Put','修改密码',64),(7402715544576496640,4,7234570580194661376,'/sys/useronline/del','Delete','退出在线用户',65),(7402715544626828288,4,7232752190589146112,'/sys/user/edit','Put','编辑用户',66),(7402715544677159936,4,7232752190656254976,'/sys/dept/edit','Put','编辑部门',67),(7402715544727491584,4,7232752190727558144,'/sys/dept/add','Post','添加部门',68),(7402715544782017536,4,7232752190794667008,'/sys/dept/list','Get','获取部门列表',69),(7402715544828154880,4,7234253747373642752,'/sys/operationlog/list','Get','获取操作日志列表',70),(7402715544870097920,4,7232752190861775872,'/sys/dept/del','Delete','删除部门',71),(7402715544907846656,4,7232752190933079040,'/sys/dept/dept_tree','Get','获取部门树',72),(7402715544958178304,4,7232752190995993600,'/sys/useronline/list','Get','获取在线用户列表',73),(7402715545000121344,4,7232752191063102464,'/sys/logininfo/list','Get','获取登录日志列表',74),(7402715545054647296,4,7253067149336286208,'/sys/user/update_role_or_dept','Post','更新用户拥有的用户部门和用户角色',75),(7402715545109173248,4,7233868069737501696,'/sys/apipermission/edit','Put','编辑api权限',76),(7402715545159504896,4,7241490807281062912,'/sys/user/login_out','Get','用户退出',77),(7402715545205642240,4,7240020747752477696,'/sys/user/refersh_token','Put','刷新token',78),(7402715545268556800,4,7235678367742071808,'/sys/job/hand_execute_job','Post','执行定时任务',79),(7402715545323082752,4,7385368752226735104,'/test/test_api/list','Get','获取列表',80),(7402715545373414400,4,7385368752235123712,'/test/test_api/del','Delete','删除TestApi',81),(7402715545444717568,4,7401293722618336256,'/test/test_api/db_read_write_test','Put','db_read_write_test',82),(7402715545495049216,4,7348333842530210816,'/wechat/wxaccounts/del','Delete','删除WxAccounts',83),(7402715545549575168,4,7385368752239318016,'/test/test_api/add','Post','添加TestApi',84),(7402715545599906816,4,7401293722630919168,'/test/test_api/db_name_test','Put','db_name_test',85),(7402715545650238464,4,7348333842559570944,'/wechat/wxaccounts/edit','Put','编辑WxAccounts',86),(7402715545704764416,4,7348333842567959552,'/wechat/wxaccounts/list','Get','获取WxAccounts列表',87),(7402715545755096064,4,7385368752243512320,'/test/test_api/edit','Put','编辑TestApi',88),(7402715545801233408,4,7348333842572153856,'/wechat/wxaccounts/add','Post','添加WxAccounts',89),(7402715545847370752,4,7348333842584736768,'/wechat/wxautoreplies/add','Post','添加WxAutoReplies',90),(7402715545893508096,4,7401293722647696384,'/test/test_api/db_index_test','Put','db_index_test',91),(7402715545939645440,4,7348333842609902592,'/wechat/wxautoreplies/del','Delete','删除WxAutoReplies',92),(7402715545989977088,4,7348333842635068416,'/wechat/wxautoreplies/list','Get','获取WxAutoReplies列表',93),(7402715546031920128,4,7401293722651890688,'/test/test_api/db_auto_test','Put','db_auto_test',94),(7402715546086446080,4,7348333843008361472,'/wechat/wxautoreplies/edit','Put','编辑WxAutoReplies',95),(7402715546136777728,4,7402694868939478016,'/test/test_data_scope/del','Delete','Delete',96),(7402715546182915072,4,7402694868947866624,'/test/test_data_scope/list','Get','list',97),(7402715546233246720,4,7348333843016750080,'/wechat/wxmenus/list','Get','获取WxMenus列表',98),(7402715546283578368,4,7348333843071276032,'/wechat/wxmenus/del','Delete','删除WxMenus',99),(7402715546329715712,4,7402694868947866625,'/test/test_data_scope/add','Post','add',100),(7402715546384241664,4,7402694868952060928,'/test/test_data_scope/edit','Put','edit',101),(7402715546426184704,4,7348333843075470336,'/wechat/wxmenus/pull_menu','Get','创建自定义菜单',102),(7402715546484904960,4,7348333843092247552,'/wechat/wxmenus/add','Post','添加WxMenus',103),(7402715546535236608,4,7348333843100636160,'/wechat/wxmenus/edit','Put','编辑WxMenus',104),(7402715546585568256,4,7348333843104830464,'/wechat/wxmessages/list','Get','获取WxMessages列表',105),(7402715546635899904,4,7348333843125801984,'/wechat/wxmessages/edit','Put','编辑WxMessages',106),(7402715546690425856,4,7348333843138384896,'/wechat/wxmessages/del','Delete','删除WxMessages',107),(7402715546740757504,4,7348333843150967808,'/wechat/wxmessages/add','Post','添加WxMessages',108),(7402715546791089152,4,7348333843159356416,'/wechat/wxusers/add','Post','添加WxUsers',109),(7402715546841420800,4,7348333843167745024,'/wechat/wxusers/edit','Put','编辑WxUsers',110),(7402715546900141056,4,7348333843180327936,'/wechat/wxusers/del','Delete','删除WxUsers',111),(7402715546954667008,4,7348333843184522240,'/wechat/wxusers/list','Get','获取WxUsers列表',112),(7402728415402300416,3,7232752187397280768,'/sys/job/add','Post','添加定时任务',1),(7402728415490380800,3,7232752187535692800,'/sys/job/validate_cron','Post','验证cron表达式',2),(7402728415574266880,3,7232752187606995968,'/sys/job/list','Get','获取定时任务列表',3),(7402728415649764352,3,7232752187674104832,'/sys/job/edit','Put','编辑定时任务',4),(7402728415712678912,3,7232752187741213696,'/sys/job/del','Delete','删除定时任务',5),(7402728415758816256,3,7232752187812516864,'/sys/roleapi/add_many_role_api_transfer','Get','添加多个角色api',6),(7402728415825925120,3,7232752187879625728,'/sys/roleapi/del','Delete','删除角色api',7),(7402728415893033984,3,7232752187946734592,'/sys/roleapi/role_api_transfer_list','Get','获取角色api所有的选择列表',8),(7402728415955948544,3,7232752188013843456,'/sys/roleapi/list','Get','获取角色api列表',9),(7402728416023057408,3,7232752188085146624,'/sys/roleapi/edit','Put','编辑角色api',10),(7402728416077583360,3,7232752188156449792,'/sys/roleapi/role_permission_list','Get','根据角色id获取api权限列表',11),(7402728416123720704,3,7232752188219364352,'/sys/roleapi/add','Post','添加角色api',12),(7402728416174052352,3,7235633673968456704,'/sys/role/get_role_depts','Get','获取角色部门',13),(7402728416228578304,3,7232752188282278912,'/sys/dicttype/list','Get','获取字典类型列表',14),(7402728416283104256,3,7252729164858299392,'/sys/role/user_role_name_list','Get','获取用户拥有的角色名称列表',15),(7402728416333435904,3,7232752188349387776,'/sys/dicttype/add','Post','添加字典类型',16),(7402728416379573248,3,7232752188412302336,'/sys/dicttype/del','Delete','删除字典类型',17),(7402728416429904896,3,7232752188479411200,'/sys/dicttype/edit','Put','编辑字典类型',18),(7402728416480236544,3,7232752188546520064,'/sys/menu/edit','Put','编辑菜单',19),(7402728416530568192,3,7234616780964926464,'/sys/cache/clear','Post','清空缓存',20),(7402728416580899840,3,7232752188617823232,'/sys/menu/list','Get','菜单列表',21),(7402728416631231488,3,7234616781027841024,'/sys/cache/list','Get','获取缓存列表',22),(7402728416681563136,3,7232752188697515008,'/sys/menu/add','Post','添加菜单',23),(7402728416731894784,3,7232752188764623872,'/sys/menu/del','Delete','删除菜单',24),(7402728416773837824,3,7232752188835927040,'/sys/menu/all_router','Get','全部路由',25),(7402728416824169472,3,7232752188898841600,'/sys/menu/tree','Get','获取菜单树',26),(7402728416887084032,3,7232752188965950464,'/sys/serverinfo/server_update','Get','更新服务器信息',27),(7402728416937415680,3,7232752189028865024,'/sys/jobinfo/add','Post','添加定时任务日志',28),(7402728416991941632,3,7232794958103483392,'/sys/dashboard/analysis/weeklyUserActivity','Get','获取仪表盘数据',29),(7402728417042273280,3,7232752189095973888,'/sys/jobinfo/list','Get','获取定时任务日志列表',30),(7402728417088410624,3,7232794958170592256,'/sys/dashboard/analysis/total','Get','获取仪表盘数据',31),(7402728417138742272,3,7232781502121939968,'/sys/user/delusers','Delete','删除用户',32),(7402728417189073920,3,7232752189158888448,'/sys/dictdata/get_by_type','Get','根据类型获取字典数据',33),(7402728417226822656,3,7232794958229312512,'/sys/dashboard/analysis/monthlySales','Get','获取仪表盘数据',34),(7402728417281348608,3,7232794958279644160,'/sys/dashboard/analysis/userAccessSource','Get','获取仪表盘数据',35),(7402728417323291648,3,7232752189221803008,'/sys/dictdata/list','Get','获取字典数据列表',36),(7402728417369428992,3,7232752189280523264,'/sys/dictdata/del','Delete','删除字典数据',37),(7402728417423954944,3,7232794958317392896,'/sys/dashboard/workplace/dynamic','Get','获取仪表盘数据',38),(7402728417486869504,3,7232794958363530240,'/sys/dashboard/workplace/total','Get','获取仪表盘数据',39),(7402728417541395456,3,7232752189347632128,'/sys/dictdata/edit','Put','编辑字典数据',40),(7402728417587532800,3,7232752189410546688,'/sys/dictdata/add','Post','添加字典数据',41),(7402728417637864448,3,7232794958401278976,'/sys/dashboard/workplace/radar','Get','获取仪表盘数据',42),(7402728417688196096,3,7232794958447416320,'/sys/dashboard/workplace/project','Get','获取仪表盘数据',43),(7402728417738527744,3,7232752189473461248,'/sys/dashboard/add','Post','添加仪表盘',44),(7402728417776276480,3,7232794958493553664,'/sys/dashboard/workplace/team','Get','获取仪表盘数据',45),(7402728417826608128,3,7232752189557347328,'/sys/dashboard/list','Get','获取仪表盘列表',46),(7402728417872745472,3,7232752189628650496,'/sys/role/list','Get','获取角色列表',47),(7402728417927271424,3,7232752189691565056,'/sys/role/get_role_menus','Get','获取角色菜单',48),(7402728417981797376,3,7232752189758673920,'/sys/role/menu','Get','获取角色菜单',49),(7402728418036323328,3,7236784152022782976,'/sys/user/fresh_token','Put','刷新token',50),(7402728418095043584,3,7232752189821588480,'/sys/role/edit','Put','编辑角色',51),(7402728418145375232,3,7232752189888697344,'/sys/role/tree','Get','获取角色树',52),(7402728418195706880,3,7232752189951611904,'/sys/role/add','Post','添加角色',53),(7402728418250232832,3,7236784153973134336,'/sys/user/update_avatar','Put','更新头像',54),(7402728418304758784,3,7232752190018720768,'/sys/role/del','Delete','删除角色',55),(7402728418355090432,3,7232752190081635328,'/sys/apipermission/list','Get','获取api权限列表',56),(7402728418405422080,3,7232752190144549888,'/sys/user/reset_password','Put','重置密码',57),(7402728418455753728,3,7232752190207464448,'/sys/user/userinfo','Get','获取用户信息',58),(7402728418506085376,3,7232752190270379008,'/sys/user/depts_roles','Get','获取用户拥有的用户部门和用户角色',59),(7402728418552222720,3,7232752190333293568,'/sys/user/del','Delete','删除用户',60),(7402728418602554368,3,7252738573697192960,'/sys/dept/user_dept_name_list','Get','获取用户拥有的部门名称列表',61),(7402728418669663232,3,7232752190396208128,'/sys/user/list','Get','获取用户列表',62),(7402728418715800576,3,7232752190459122688,'/sys/user/add','Post','添加用户',63),(7402728418757743616,3,7232752190526231552,'/sys/user/change_password','Put','修改密码',64),(7402728418803880960,3,7234570580194661376,'/sys/useronline/del','Delete','退出在线用户',65),(7402728418854212608,3,7232752190589146112,'/sys/user/edit','Put','编辑用户',66),(7402728418908738560,3,7232752190656254976,'/sys/dept/edit','Put','编辑部门',67),(7402728418963264512,3,7232752190727558144,'/sys/dept/add','Post','添加部门',68),(7402728419017790464,3,7232752190794667008,'/sys/dept/list','Get','获取部门列表',69),(7402728419072316416,3,7234253747373642752,'/sys/operationlog/list','Get','获取操作日志列表',70),(7402728419122648064,3,7232752190861775872,'/sys/dept/del','Delete','删除部门',71),(7402728419177174016,3,7232752190933079040,'/sys/dept/dept_tree','Get','获取部门树',72),(7402728419227505664,3,7232752190995993600,'/sys/useronline/list','Get','获取在线用户列表',73),(7402728419265254400,3,7232752191063102464,'/sys/logininfo/list','Get','获取登录日志列表',74),(7402728419328168960,3,7253067149336286208,'/sys/user/update_role_or_dept','Post','更新用户拥有的用户部门和用户角色',75),(7402728419382694912,3,7233868069737501696,'/sys/apipermission/edit','Put','编辑api权限',76),(7402728419433026560,3,7241490807281062912,'/sys/user/login_out','Get','用户退出',77),(7402728419479163904,3,7240020747752477696,'/sys/user/refersh_token','Put','刷新token',78),(7402728419529495552,3,7235678367742071808,'/sys/job/hand_execute_job','Post','执行定时任务',79),(7402728419584021504,3,7385368752226735104,'/test/test_api/list','Get','获取列表',80),(7402728419638547456,3,7385368752235123712,'/test/test_api/del','Delete','删除TestApi',81),(7402728419688879104,3,7401293722618336256,'/test/test_api/db_read_write_test','Put','db_read_write_test',82),(7402728419751793664,3,7348333842530210816,'/wechat/wxaccounts/del','Delete','删除WxAccounts',83),(7402728419818902528,3,7385368752239318016,'/test/test_api/add','Post','添加TestApi',84),(7402728419873428480,3,7401293722630919168,'/test/test_api/db_name_test','Put','db_name_test',85),(7402728419940537344,3,7348333842559570944,'/wechat/wxaccounts/edit','Put','编辑WxAccounts',86),(7402728419999257600,3,7348333842567959552,'/wechat/wxaccounts/list','Get','获取WxAccounts列表',87),(7402728420053783552,3,7385368752243512320,'/test/test_api/edit','Put','编辑TestApi',88),(7402728420099920896,3,7348333842572153856,'/wechat/wxaccounts/add','Post','添加WxAccounts',89),(7402728420150252544,3,7348333842584736768,'/wechat/wxautoreplies/add','Post','添加WxAutoReplies',90),(7402728420192195584,3,7401293722647696384,'/test/test_api/db_index_test','Put','db_index_test',91),(7402728420242527232,3,7348333842609902592,'/wechat/wxautoreplies/del','Delete','删除WxAutoReplies',92),(7402728420292858880,3,7348333842635068416,'/wechat/wxautoreplies/list','Get','获取WxAutoReplies列表',93),(7402728420351579136,3,7401293722651890688,'/test/test_api/db_auto_test','Put','db_auto_test',94),(7402728420406105088,3,7348333843008361472,'/wechat/wxautoreplies/edit','Put','编辑WxAutoReplies',95),(7402728420460631040,3,7402694868939478016,'/test/test_data_scope/del','Delete','Delete',96),(7402728420506768384,3,7402694868947866624,'/test/test_data_scope/list','Get','list',97),(7402728420548711424,3,7348333843016750080,'/wechat/wxmenus/list','Get','获取WxMenus列表',98),(7402728420624208896,3,7348333843071276032,'/wechat/wxmenus/del','Delete','删除WxMenus',99),(7402728420682929152,3,7402694868947866625,'/test/test_data_scope/add','Post','add',100),(7402728420733260800,3,7402694868952060928,'/test/test_data_scope/edit','Put','edit',101),(7402728420796175360,3,7348333843075470336,'/wechat/wxmenus/pull_menu','Get','创建自定义菜单',102),(7402728420854895616,3,7348333843092247552,'/wechat/wxmenus/add','Post','添加WxMenus',103),(7402728420909421568,3,7348333843100636160,'/wechat/wxmenus/edit','Put','编辑WxMenus',104),(7402728420959753216,3,7348333843104830464,'/wechat/wxmessages/list','Get','获取WxMessages列表',105),(7402728421018473472,3,7348333843125801984,'/wechat/wxmessages/edit','Put','编辑WxMessages',106),(7402728421072999424,3,7348333843138384896,'/wechat/wxmessages/del','Delete','删除WxMessages',107),(7402728421127525376,3,7348333843150967808,'/wechat/wxmessages/add','Post','添加WxMessages',108),(7402728421194634240,3,7348333843159356416,'/wechat/wxusers/add','Post','添加WxUsers',109),(7402728421249160192,3,7348333843167745024,'/wechat/wxusers/edit','Put','编辑WxUsers',110),(7402728421303686144,3,7348333843180327936,'/wechat/wxusers/del','Delete','删除WxUsers',111),(7402728421358212096,3,7348333843184522240,'/wechat/wxusers/list','Get','获取WxUsers列表',112),(7403405998959793152,1,7232752187397280768,'/sys/job/add','Post','添加定时任务',1),(7403405999001736192,1,7232752187535692800,'/sys/job/validate_cron','Post','验证cron表达式',2),(7403405999039484928,1,7232752187606995968,'/sys/job/list','Get','获取定时任务列表',3),(7403405999068845056,1,7232752187674104832,'/sys/job/edit','Put','编辑定时任务',4),(7403405999094010880,1,7232752187741213696,'/sys/job/del','Delete','删除定时任务',5),(7403405999119176704,1,7232752187812516864,'/sys/roleapi/add_many_role_api_transfer','Get','添加多个角色api',6),(7403405999148536832,1,7232752187879625728,'/sys/roleapi/del','Delete','删除角色api',7),(7403405999182091264,1,7232752187946734592,'/sys/roleapi/role_api_transfer_list','Get','获取角色api所有的选择列表',8),(7403405999215645696,1,7232752188013843456,'/sys/roleapi/list','Get','获取角色api列表',9),(7403405999245005824,1,7232752188085146624,'/sys/roleapi/edit','Put','编辑角色api',10),(7403405999274365952,1,7232752188156449792,'/sys/roleapi/role_permission_list','Get','根据角色id获取api权限列表',11),(7403405999303726080,1,7232752188219364352,'/sys/roleapi/add','Post','添加角色api',12),(7403405999358252032,1,7235633673968456704,'/sys/role/get_role_depts','Get','获取角色部门',13),(7403405999391806464,1,7232752188282278912,'/sys/dicttype/list','Get','获取字典类型列表',14),(7403405999425360896,1,7252729164858299392,'/sys/role/user_role_name_list','Get','获取用户拥有的角色名称列表',15),(7403405999458915328,1,7232752188349387776,'/sys/dicttype/add','Post','添加字典类型',16),(7403405999488275456,1,7232752188412302336,'/sys/dicttype/del','Delete','删除字典类型',17),(7403405999517635584,1,7232752188479411200,'/sys/dicttype/edit','Put','编辑字典类型',18),(7403405999551190016,1,7232752188546520064,'/sys/menu/edit','Put','编辑菜单',19),(7403405999584744448,1,7234616780964926464,'/sys/cache/clear','Post','清空缓存',20),(7403405999618298880,1,7232752188617823232,'/sys/menu/list','Get','菜单列表',21),(7403405999651853312,1,7234616781027841024,'/sys/cache/list','Get','获取缓存列表',22),(7403405999677019136,1,7232752188697515008,'/sys/menu/add','Post','添加菜单',23),(7403405999710573568,1,7232752188764623872,'/sys/menu/del','Delete','删除菜单',24),(7403405999739933696,1,7232752188835927040,'/sys/menu/all_router','Get','全部路由',25),(7403405999769293824,1,7232752188898841600,'/sys/menu/tree','Get','获取菜单树',26),(7403405999802848256,1,7232752188965950464,'/sys/serverinfo/server_update','Get','更新服务器信息',27),(7403405999840596992,1,7232752189028865024,'/sys/jobinfo/add','Post','添加定时任务日志',28),(7403405999878345728,1,7232794958103483392,'/sys/dashboard/analysis/weeklyUserActivity','Get','获取仪表盘数据',29),(7403405999911900160,1,7232752189095973888,'/sys/jobinfo/list','Get','获取定时任务日志列表',30),(7403405999941260288,1,7232794958170592256,'/sys/dashboard/analysis/total','Get','获取仪表盘数据',31),(7403405999966426112,1,7232781502121939968,'/sys/user/delusers','Delete','删除用户',32),(7403405999999980544,1,7232752189158888448,'/sys/dictdata/get_by_type','Get','根据类型获取字典数据',33),(7403406000033534976,1,7232794958229312512,'/sys/dashboard/analysis/monthlySales','Get','获取仪表盘数据',34),(7403406000067089408,1,7232794958279644160,'/sys/dashboard/analysis/userAccessSource','Get','获取仪表盘数据',35),(7403406000100643840,1,7232752189221803008,'/sys/dictdata/list','Get','获取字典数据列表',36),(7403406000125809664,1,7232752189280523264,'/sys/dictdata/del','Delete','删除字典数据',37),(7403406000155169792,1,7232794958317392896,'/sys/dashboard/workplace/dynamic','Get','获取仪表盘数据',38),(7403406000188724224,1,7232794958363530240,'/sys/dashboard/workplace/total','Get','获取仪表盘数据',39),(7403406000213890048,1,7232752189347632128,'/sys/dictdata/edit','Put','编辑字典数据',40),(7403406000251638784,1,7232752189410546688,'/sys/dictdata/add','Post','添加字典数据',41),(7403406000280998912,1,7232794958401278976,'/sys/dashboard/workplace/radar','Get','获取仪表盘数据',42),(7403406000310359040,1,7232794958447416320,'/sys/dashboard/workplace/project','Get','获取仪表盘数据',43),(7403406000339719168,1,7232752189473461248,'/sys/dashboard/add','Post','添加仪表盘',44),(7403406000373273600,1,7232794958493553664,'/sys/dashboard/workplace/team','Get','获取仪表盘数据',45),(7403406000402633728,1,7232752189557347328,'/sys/dashboard/list','Get','获取仪表盘列表',46),(7403406000431993856,1,7232752189628650496,'/sys/role/list','Get','获取角色列表',47),(7403406000461353984,1,7232752189691565056,'/sys/role/get_role_menus','Get','获取角色菜单',48),(7403406000490714112,1,7232752189758673920,'/sys/role/menu','Get','获取角色菜单',49),(7403406000520074240,1,7236784152022782976,'/sys/user/fresh_token','Put','刷新token',50),(7403406000553628672,1,7232752189821588480,'/sys/role/edit','Put','编辑角色',51),(7403406000578794496,1,7232752189888697344,'/sys/role/tree','Get','获取角色树',52),(7403406000616543232,1,7232752189951611904,'/sys/role/add','Post','添加角色',53),(7403406000645903360,1,7236784153973134336,'/sys/user/update_avatar','Put','更新头像',54),(7403406000675263488,1,7232752190018720768,'/sys/role/del','Delete','删除角色',55),(7403406000708817920,1,7232752190081635328,'/sys/apipermission/list','Get','获取api权限列表',56),(7403406000738178048,1,7232752190144549888,'/sys/user/reset_password','Put','重置密码',57),(7403406000767538176,1,7232752190207464448,'/sys/user/userinfo','Get','获取用户信息',58),(7403406000796898304,1,7232752190270379008,'/sys/user/depts_roles','Get','获取用户拥有的用户部门和用户角色',59),(7403406000838841344,1,7232752190333293568,'/sys/user/del','Delete','删除用户',60),(7403406000868201472,1,7252738573697192960,'/sys/dept/user_dept_name_list','Get','获取用户拥有的部门名称列表',61),(7403406000901755904,1,7232752190396208128,'/sys/user/list','Get','获取用户列表',62),(7403406000931116032,1,7232752190459122688,'/sys/user/add','Post','添加用户',63),(7403406000960476160,1,7232752190526231552,'/sys/user/change_password','Put','修改密码',64),(7403406000994030592,1,7234570580194661376,'/sys/useronline/del','Delete','退出在线用户',65),(7403406001019196416,1,7232752190589146112,'/sys/user/edit','Put','编辑用户',66),(7403406001048556544,1,7232752190656254976,'/sys/dept/edit','Put','编辑部门',67),(7403406001077916672,1,7232752190727558144,'/sys/dept/add','Post','添加部门',68),(7403406001107276800,1,7232752190794667008,'/sys/dept/list','Get','获取部门列表',69),(7403406001136636928,1,7234253747373642752,'/sys/operationlog/list','Get','获取操作日志列表',70),(7403406001165997056,1,7232752190861775872,'/sys/dept/del','Delete','删除部门',71),(7403406001199551488,1,7232752190933079040,'/sys/dept/dept_tree','Get','获取部门树',72),(7403406001233105920,1,7232752190995993600,'/sys/useronline/list','Get','获取在线用户列表',73),(7403406001258271744,1,7232752191063102464,'/sys/logininfo/list','Get','获取登录日志列表',74),(7403406001291826176,1,7253067149336286208,'/sys/user/update_role_or_dept','Post','更新用户拥有的用户部门和用户角色',75),(7403406001321186304,1,7233868069737501696,'/sys/apipermission/edit','Put','编辑api权限',76),(7403406001350546432,1,7241490807281062912,'/sys/user/login_out','Get','用户退出',77),(7403406001375712256,1,7240020747752477696,'/sys/user/refersh_token','Put','刷新token',78),(7403406001405072384,1,7235678367742071808,'/sys/job/hand_execute_job','Post','执行定时任务',79),(7403406001434432512,1,7385368752226735104,'/test/test_api/list','Get','获取列表',80),(7403406001463792640,1,7385368752235123712,'/test/test_api/del','Delete','删除TestApi',81),(7403406001493152768,1,7401293722618336256,'/test/test_api/db_read_write_test','Put','db_read_write_test',82),(7403406001522512896,1,7348333842530210816,'/wechat/wxaccounts/del','Delete','删除WxAccounts',83),(7403406001560261632,1,7385368752239318016,'/test/test_api/add','Post','添加TestApi',84),(7403406001589621760,1,7401293722630919168,'/test/test_api/db_name_test','Put','db_name_test',85),(7403406001623176192,1,7348333842559570944,'/wechat/wxaccounts/edit','Put','编辑WxAccounts',86),(7403406001652536320,1,7348333842567959552,'/wechat/wxaccounts/list','Get','获取WxAccounts列表',87),(7403406001681896448,1,7385368752243512320,'/test/test_api/edit','Put','编辑TestApi',88),(7403406001711256576,1,7348333842572153856,'/wechat/wxaccounts/add','Post','添加WxAccounts',89),(7403406001740616704,1,7348333842584736768,'/wechat/wxautoreplies/add','Post','添加WxAutoReplies',90),(7403406001769976832,1,7401293722647696384,'/test/test_api/db_index_test','Put','db_index_test',91),(7403406001795142656,1,7348333842609902592,'/wechat/wxautoreplies/del','Delete','删除WxAutoReplies',92),(7403406001858057216,1,7348333842635068416,'/wechat/wxautoreplies/list','Get','获取WxAutoReplies列表',93),(7403406001887417344,1,7401293722651890688,'/test/test_api/db_auto_test','Put','db_auto_test',94),(7403406001912583168,1,7348333843008361472,'/wechat/wxautoreplies/edit','Put','编辑WxAutoReplies',95),(7403406001937748992,1,7402694868939478016,'/test/test_data_scope/del','Delete','Delete',96),(7403406001971303424,1,7402694868947866624,'/test/test_data_scope/list','Get','list',97),(7403406002004857856,1,7348333843016750080,'/wechat/wxmenus/list','Get','获取WxMenus列表',98),(7403406002042606592,1,7348333843071276032,'/wechat/wxmenus/del','Delete','删除WxMenus',99),(7403406002071966720,1,7402694868947866625,'/test/test_data_scope/add','Post','add',100),(7403406002113909760,1,7402694868952060928,'/test/test_data_scope/edit','Put','edit',101),(7403406002139075584,1,7348333843075470336,'/wechat/wxmenus/pull_menu','Get','创建自定义菜单',102),(7403406002168435712,1,7348333843092247552,'/wechat/wxmenus/add','Post','添加WxMenus',103),(7403406002197795840,1,7348333843100636160,'/wechat/wxmenus/edit','Put','编辑WxMenus',104),(7403406002227155968,1,7348333843104830464,'/wechat/wxmessages/list','Get','获取WxMessages列表',105),(7403406002252321792,1,7348333843125801984,'/wechat/wxmessages/edit','Put','编辑WxMessages',106),(7403406002285876224,1,7348333843138384896,'/wechat/wxmessages/del','Delete','删除WxMessages',107),(7403406002319430656,1,7348333843150967808,'/wechat/wxmessages/add','Post','添加WxMessages',108),(7403406002348790784,1,7348333843159356416,'/wechat/wxusers/add','Post','添加WxUsers',109),(7403406002378150912,1,7348333843167745024,'/wechat/wxusers/edit','Put','编辑WxUsers',110),(7403406002411705344,1,7348333843180327936,'/wechat/wxusers/del','Delete','删除WxUsers',111),(7403406002441065472,1,7348333843184522240,'/wechat/wxusers/list','Get','获取WxUsers列表',112);
/*!40000 ALTER TABLE `sys_role_api` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_role_dept`
--

DROP TABLE IF EXISTS `sys_role_dept`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_role_dept` (
  `role_id` bigint(20) NOT NULL COMMENT '角色ID',
  `dept_id` bigint(20) NOT NULL COMMENT '部门ID',
  PRIMARY KEY (`role_id`,`dept_id`),
  KEY `dept_id` (`dept_id`),
  CONSTRAINT `sys_role_dept_ibfk_1` FOREIGN KEY (`role_id`) REFERENCES `sys_role` (`role_id`),
  CONSTRAINT `sys_role_dept_ibfk_2` FOREIGN KEY (`dept_id`) REFERENCES `sys_dept` (`dept_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='角色和部门关联表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_role_dept`
--

LOCK TABLES `sys_role_dept` WRITE;
/*!40000 ALTER TABLE `sys_role_dept` DISABLE KEYS */;
INSERT INTO `sys_role_dept` VALUES (7222985345490620417,101),(7222985345490620417,103),(7222985345490620417,104);
/*!40000 ALTER TABLE `sys_role_dept` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_role_menu`
--

DROP TABLE IF EXISTS `sys_role_menu`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_role_menu` (
  `role_id` bigint(20) NOT NULL COMMENT '角色ID',
  `menu_id` bigint(20) NOT NULL COMMENT '菜单ID',
  PRIMARY KEY (`role_id`,`menu_id`),
  KEY `menu_id` (`menu_id`),
  CONSTRAINT `sys_role_menu_ibfk_1` FOREIGN KEY (`menu_id`) REFERENCES `sys_menu` (`id`),
  CONSTRAINT `sys_role_menu_ibfk_2` FOREIGN KEY (`role_id`) REFERENCES `sys_role` (`role_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='角色和菜单关联表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_role_menu`
--

LOCK TABLES `sys_role_menu` WRITE;
/*!40000 ALTER TABLE `sys_role_menu` DISABLE KEYS */;
INSERT INTO `sys_role_menu` VALUES (1,1),(3,1),(4,1),(7222985345490620417,1),(1,2),(3,2),(4,2),(7222985345490620417,2),(1,3),(7222985345490620417,3),(1,4),(3,4),(7222985345490620417,4),(1,5),(3,5),(7222985345490620417,5),(1,6),(3,6),(4,6),(7222985345490620417,6),(1,7),(4,7),(7222985345490620417,7),(1,8),(4,8),(7222985345490620417,8),(3,7224471859710005249),(3,7225439414641627136),(3,7228454585970593792),(3,7402695499167208448),(4,7402695499167208448),(7222985345490620417,7402695499167208448),(3,7402702074216879104),(4,7402702074216879104);
/*!40000 ALTER TABLE `sys_role_menu` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_user`
--

DROP TABLE IF EXISTS `sys_user`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_user` (
  `id` bigint(20) NOT NULL COMMENT '用户ID',
  `dept_id` bigint(20) NOT NULL COMMENT '部门ID',
  `role_id` bigint(20) NOT NULL COMMENT '激活的ID',
  `user_name` varchar(30) NOT NULL COMMENT '用户账号',
  `nick_name` varchar(30) NOT NULL COMMENT '用户昵称',
  `user_type` varchar(10) DEFAULT 'sys_user' COMMENT '用户类型（sys_user系统用户）',
  `email` varchar(50) DEFAULT '' COMMENT '用户邮箱',
  `phonenumber` varchar(11) DEFAULT '' COMMENT '手机号码',
  `sex` char(1) DEFAULT '0' COMMENT '用户性别（0男 1女 2未知）',
  `avatar` varchar(256) DEFAULT NULL COMMENT '头像地址',
  `password` varchar(100) NOT NULL DEFAULT '' COMMENT '密码',
  `status` char(1) DEFAULT '0' COMMENT '帐号状态（0正常 1停用）',
  `login_ip` varchar(128) DEFAULT '' COMMENT '最后登录IP',
  `login_date` datetime DEFAULT NULL COMMENT '最后登录时间',
  `create_dept` bigint(20) DEFAULT NULL COMMENT '创建部门',
  `create_by` bigint(20) DEFAULT NULL COMMENT '创建者',
  `created_at` datetime DEFAULT NULL COMMENT '创建时间',
  `update_by` bigint(20) DEFAULT NULL COMMENT '更新者',
  `updated_at` datetime DEFAULT NULL COMMENT '更新时间',
  `deleted_at` datetime DEFAULT NULL COMMENT '删除',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户信息表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_user`
--

LOCK TABLES `sys_user` WRITE;
/*!40000 ALTER TABLE `sys_user` DISABLE KEYS */;
INSERT INTO `sys_user` VALUES (1,100,1,'admin','管理员','sys_user','willmagic@qq.com','18588719996','1','static/2024-10/25_7255598825434158080.png','$argon2id$v=19$m=19456,t=2,p=1$M6+FVFGkM/zIAC7VZgGoLQ$zSAK9IIFN28ZsskO5s1eKKNv2g4yGw2b+CKjRa1cLvo','0','127.0.0.1','2024-07-03 01:17:31',103,1,'2024-07-03 01:17:31',NULL,NULL,NULL,'管理员'),(3,104,3,'test','测试1','sys_user','test@qq.com','15888888889','0',NULL,'$argon2id$v=19$m=19456,t=2,p=1$mO97IyGb5zSmoGrmB3ET1A$x8CKk5Z33siO3zVtMUm2AjMQsRrfNISa35S+sGmksf4','0','127.0.0.1','2024-07-03 01:17:31',103,1,'2024-07-03 01:17:31',3,'2024-07-03 01:17:31',NULL,NULL),(4,102,3,'test1','测试q','sys_user','test1@qq.com','15888888887','0',NULL,'$2a$10$b8yUzN0C71sbz.PhNOCgJe.Tu1yWC3RNrTyjSQ8p1W0.aaUXUJ.Ne','0','127.0.0.1','2024-07-03 01:17:31',103,1,'2024-07-03 01:17:31',4,'2024-07-03 01:17:31','2024-08-24 00:21:07',NULL),(5,102,3,'test2','测试2','sys_user','test2@qq.com','15888888886','0',NULL,'$2a$10$b8yUzN0C71sbz.PhNOCgJe.Tu1yWC3RNrTyjSQ8p1W0.aaUXUJ.Ne','0','127.0.0.1','2024-07-03 01:17:31',103,1,'2024-07-03 01:17:31',4,'2024-07-03 01:17:31',NULL,NULL),(7222295451336314881,103,4,'test3','测试3','sys_user','test3@qq.com','0588445337','1',NULL,'$argon2id$v=19$m=19456,t=2,p=1$2ekLWIma6X91vcafwSnWmA$/l1xAacSwe3c/urIwRNTCb3l0hf1zDmUSdpAsm1V/ok','0','','2024-07-03 01:17:31',NULL,NULL,'2024-07-03 01:17:31',NULL,NULL,NULL,NULL),(7222299781468655617,104,3,'test4','测试4','sys_user','test4@qq.com','0588445337','0',NULL,'$argon2id$v=19$m=19456,t=2,p=1$JJh+GICj7Ii7dnexWgNrZQ$h/MU+GhTX5O9SMu7p9S899QUBWLM8QPo3lpFk7KGars','0','','2024-07-03 01:17:31',NULL,NULL,'2024-07-03 01:17:31',NULL,NULL,NULL,'1'),(7223731680347033600,7223731313261547520,7222985345490620417,'test5','test5','sys_user','test5@qq.com','0588445337',NULL,NULL,'$argon2id$v=19$m=19456,t=2,p=1$VsofXU4ao39GasijfbQvjw$sASrh2idw664BmrRN8XPguppliyXbFmYJhvclNtSi3w',NULL,'','2024-07-03 01:17:31',NULL,NULL,'2024-07-03 01:17:31',NULL,NULL,NULL,NULL),(7402727913407026176,101,3,'test6','test6','sys_user',NULL,NULL,NULL,NULL,'$argon2id$v=19$m=19456,t=2,p=1$gD8LZFl1wYA5iG8kf97ELw$pl5iL7eE7U5p97o6mu92vbft0VwsjbTa3iATXSKkJRw','0','',NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL);
/*!40000 ALTER TABLE `sys_user` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_user_dept`
--

DROP TABLE IF EXISTS `sys_user_dept`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_user_dept` (
  `user_id` bigint(20) NOT NULL,
  `dept_id` bigint(20) NOT NULL,
  PRIMARY KEY (`user_id`,`dept_id`),
  KEY `dept_id` (`dept_id`),
  CONSTRAINT `sys_user_dept_ibfk_1` FOREIGN KEY (`user_id`) REFERENCES `sys_user` (`id`),
  CONSTRAINT `sys_user_dept_ibfk_2` FOREIGN KEY (`dept_id`) REFERENCES `sys_dept` (`dept_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_user_dept`
--

LOCK TABLES `sys_user_dept` WRITE;
/*!40000 ALTER TABLE `sys_user_dept` DISABLE KEYS */;
INSERT INTO `sys_user_dept` VALUES (1,100),(7402727913407026176,101),(1,103),(7222295451336314881,103),(3,104),(7222299781468655617,104),(7223731680347033600,7223731313261547520);
/*!40000 ALTER TABLE `sys_user_dept` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_user_post`
--

DROP TABLE IF EXISTS `sys_user_post`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_user_post` (
  `user_id` bigint(20) NOT NULL COMMENT '用户ID',
  `post_id` bigint(20) NOT NULL COMMENT '岗位ID',
  PRIMARY KEY (`user_id`,`post_id`),
  KEY `post_id` (`post_id`),
  CONSTRAINT `sys_user_post_ibfk_1` FOREIGN KEY (`user_id`) REFERENCES `sys_user` (`id`),
  CONSTRAINT `sys_user_post_ibfk_2` FOREIGN KEY (`post_id`) REFERENCES `sys_post` (`post_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户与岗位关联表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_user_post`
--

LOCK TABLES `sys_user_post` WRITE;
/*!40000 ALTER TABLE `sys_user_post` DISABLE KEYS */;
INSERT INTO `sys_user_post` VALUES (1,1);
/*!40000 ALTER TABLE `sys_user_post` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_user_role`
--

DROP TABLE IF EXISTS `sys_user_role`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_user_role` (
  `user_id` bigint(20) NOT NULL COMMENT '用户ID',
  `role_id` bigint(20) NOT NULL COMMENT '角色ID',
  PRIMARY KEY (`user_id`,`role_id`),
  KEY `role_id` (`role_id`),
  CONSTRAINT `sys_user_role_ibfk_1` FOREIGN KEY (`role_id`) REFERENCES `sys_role` (`role_id`),
  CONSTRAINT `sys_user_role_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `sys_user` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户和角色关联表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_user_role`
--

LOCK TABLES `sys_user_role` WRITE;
/*!40000 ALTER TABLE `sys_user_role` DISABLE KEYS */;
INSERT INTO `sys_user_role` VALUES (1,1),(1,3),(3,3),(7222299781468655617,3),(7402727913407026176,3),(4,4),(7222295451336314881,4),(7223731680347033600,7222985345490620417);
/*!40000 ALTER TABLE `sys_user_role` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_white_jwt`
--

DROP TABLE IF EXISTS `sys_white_jwt`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_white_jwt` (
  `jid` bigint(20) NOT NULL,
  `uid` bigint(20) NOT NULL,
  `token_id` bigint(20) NOT NULL,
  `info_id` bigint(20) NOT NULL,
  `token_expr` bigint(20) NOT NULL,
  `created_at` datetime NOT NULL,
  `update_at` datetime NOT NULL,
  `deleted_at` datetime DEFAULT NULL,
  PRIMARY KEY (`jid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='jwt白名单';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_white_jwt`
--

LOCK TABLES `sys_white_jwt` WRITE;
/*!40000 ALTER TABLE `sys_white_jwt` DISABLE KEYS */;
INSERT INTO `sys_white_jwt` VALUES (7338250336651351040,1,7338250336584242176,7338250336584242177,1750180018,'2025-06-11 01:06:58','2025-06-11 01:06:58',NULL),(7348334080116560896,1,7348334080074617856,7348334080074617857,1752584170,'2025-07-08 20:56:10','2025-07-08 20:56:10',NULL),(7350937685579437056,1,7350937685495550976,7350937685495550977,1753204918,'2025-07-16 01:21:58','2025-07-16 01:21:58',NULL),(7351264641399297024,1,7351264641348965376,7351264641348965377,1753282870,'2025-07-16 23:01:10','2025-07-16 23:01:10',NULL),(7395848975468631040,1,7395848975426688000,7395848975426688001,1763912603,'2025-11-16 23:43:24','2025-11-16 23:43:24',NULL),(7402412372016403456,1,7402412371378869248,7402412371378869249,1765477439,'2025-12-05 02:24:00','2025-12-05 02:24:00',NULL),(7402695033029039104,1,7402695032978707456,7402695032978707457,1765544831,'2025-12-05 21:07:11','2025-12-05 21:07:11',NULL),(7402709166579422208,1,7402709166432621568,7402709166432621569,1765548200,'2025-12-05 22:03:21','2025-12-05 22:03:21',NULL),(7402714693157950464,7222295451336314881,7402714693057287168,7402714693057287169,1765549518,'2025-12-05 22:25:19','2025-12-05 22:25:19',NULL),(7402715852635542528,7222295451336314881,7402715852585210880,7402715852585210881,1765549795,'2025-12-05 22:29:55','2025-12-05 22:29:55',NULL),(7402728157268055040,7402727913407026176,7402728157159003136,7402728157159003137,1765552728,'2025-12-05 23:18:49','2025-12-05 23:18:49',NULL),(7402749118914663424,1,7402749118885303296,7402749118889497600,1765557726,'2025-12-06 00:42:06','2025-12-06 00:42:06',NULL),(7402749256496223232,7222295451336314881,7402749256399754240,7402749256399754241,1765557759,'2025-12-06 00:42:39','2025-12-06 00:42:39',NULL),(7403508110926451712,1,7403508110842565632,7403508110842565633,1765738684,'2025-12-08 02:58:04','2025-12-08 02:58:04',NULL),(7403508317714027520,7402727913407026176,7403508317600781312,7403508317604975616,1765738733,'2025-12-08 02:58:53','2025-12-08 02:58:53',NULL);
/*!40000 ALTER TABLE `sys_white_jwt` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `test_api`
--

DROP TABLE IF EXISTS `test_api`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `test_api` (
  `id` bigint(20) NOT NULL,
  `name` varchar(255) NOT NULL,
  `age` int(3) NOT NULL,
  `email` varchar(255) NOT NULL,
  `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `test_api`
--

LOCK TABLES `test_api` WRITE;
/*!40000 ALTER TABLE `test_api` DISABLE KEYS */;
INSERT INTO `test_api` VALUES (7402976005653959680,'韶颖',86,'uclgm3.tny@139.com','2025-12-06 15:43:40','2025-12-06 15:43:40');
/*!40000 ALTER TABLE `test_api` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `test_data_scope`
--

DROP TABLE IF EXISTS `test_data_scope`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `test_data_scope` (
  `id` bigint(20) NOT NULL,
  `title` varchar(200) CHARACTER SET utf8mb4 NOT NULL,
  `content` text CHARACTER SET utf8mb4,
  `dept_id` bigint(20) NOT NULL,
  `owner_id` bigint(20) NOT NULL,
  `status` char(1) CHARACTER SET utf8mb4 NOT NULL DEFAULT '0',
  `created_at` datetime NOT NULL,
  `updated_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  `deleted_at` datetime DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `idx_test_data_scope_dept_id` (`dept_id`),
  KEY `idx_test_data_scope_owner_id` (`owner_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `test_data_scope`
--

LOCK TABLES `test_data_scope` WRITE;
/*!40000 ALTER TABLE `test_data_scope` DISABLE KEYS */;
INSERT INTO `test_data_scope` VALUES (7402711480572744704,'胡说并列于长略微整齐耶耶耶渐渐画渐渐反正','cillum anim do nulla magna',103,7222295451336314881,'0','2025-12-05 22:12:33','2025-12-08 01:19:17',NULL),(7402712010544026624,'攥广告此外不像嗯油门却一齐尽管直到','incididunt laboris ipsum irure',72,35,'0','2025-12-05 22:14:39','2025-12-08 01:19:17',NULL),(7402712077673862144,'摘擅自共枯萎笔直长椅','elit sed velit quis',103,7222295451336314881,'0','2025-12-05 22:14:55','2025-12-08 01:19:17',NULL),(7402712087106851840,'出生写搜一同面对甚至拨多轻松','ea',55,72,'0','2025-12-05 22:14:57','2025-12-08 01:19:17',NULL),(7402712096971854848,'耶耶耶统统坐抄勇敢哎呀关于揽','adipisicing',56,89,'0','2025-12-05 22:15:00','2025-12-08 01:19:17',NULL),(7402712106258043904,'特地大哎哟朝向以呼必定','veniam',33,15,'0','2025-12-05 22:15:02','2025-12-08 01:19:17',NULL),(7402749020059112448,'特地大哎哟朝向以呼必定','veniam',33,15,'0','2025-12-06 00:41:43','2025-12-08 01:19:17',NULL),(7402749295490667520,'特地大哎哟朝向以呼必定','veniam',103,7222295451336314881,'0','2025-12-06 00:42:48','2025-12-08 01:19:17',NULL),(7402749517151245312,'哦糟糕嘘除了','in consequat exercitation velit',103,7222295451336314881,'0','2025-12-06 00:43:41','2025-12-08 01:19:17',NULL),(7403506317010048000,'fgdfsg ','sdfgsdfg123',103,7222295451336314881,'0','2025-12-08 02:50:56','2025-12-08 02:56:49',NULL),(7403508171047605248,'955','456',100,1,'0','2025-12-08 02:58:19','2025-12-08 02:58:19',NULL),(7403508420336063488,'测试6','测试6的数据',101,7402727913407026176,'0','2025-12-08 02:59:18','2025-12-08 02:59:18',NULL);
/*!40000 ALTER TABLE `test_data_scope` ENABLE KEYS */;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2025-12-08  3:34:30
