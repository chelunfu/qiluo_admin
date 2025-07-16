-- phpMyAdmin SQL Dump
-- version 5.2.0
-- https://www.phpmyadmin.net/
--
-- 主机： localhost
-- 生成日期： 2024-11-21 22:16:26
-- 服务器版本： 5.7.41-log
-- PHP 版本： 7.4.33

SET SQL_MODE = "NO_AUTO_VALUE_ON_ZERO";
START TRANSACTION;
SET time_zone = "+00:00";


/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;

--
-- 数据库： `backadmin`
--

-- --------------------------------------------------------

--
-- 表的结构 `seaql_migrations`
--

CREATE TABLE `seaql_migrations` (
  `version` varchar(255) NOT NULL,
  `applied_at` bigint(20) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

--
-- 转存表中的数据 `seaql_migrations`
--

INSERT INTO `seaql_migrations` (`version`, `applied_at`) VALUES
('m20220101_000001_create_table', 1723226505);

-- --------------------------------------------------------

--
-- 表的结构 `sys_api_permission`
--

CREATE TABLE `sys_api_permission` (
  `id` bigint(20) NOT NULL,
  `api` char(255) NOT NULL,
  `method` char(10) NOT NULL,
  `apiname` char(20) NOT NULL,
  `logcache` char(1) NOT NULL DEFAULT '0',
  `remark` varchar(512) DEFAULT NULL,
  `sort` int(11) NOT NULL,
  `created_at` datetime NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

--
-- 转存表中的数据 `sys_api_permission`
--

INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES
(7232752187397280768, '/sys/job/add', 'Post', '添加定时任务', '0', '但凡', 1, '2024-08-23 22:15:00'),
(7232752187535692800, '/sys/job/validate_cron', 'Post', '验证cron表达式', '0', NULL, 3, '2024-08-23 22:15:00'),
(7232752187606995968, '/sys/job/list', 'Get', '获取定时任务列表', '0', NULL, 3, '2024-08-23 22:15:00'),
(7232752187674104832, '/sys/job/edit', 'Put', '编辑定时任务', '0', NULL, 4, '2024-08-23 22:15:00'),
(7232752187741213696, '/sys/job/del', 'Delete', '删除定时任务', '0', NULL, 5, '2024-08-23 22:15:00'),
(7232752187812516864, '/sys/roleapi/add_many_role_api_transfer', 'Get', '添加多个角色api', '0', NULL, 6, '2024-08-23 22:15:00'),
(7232752187879625728, '/sys/roleapi/del', 'Delete', '删除角色api', '0', NULL, 7, '2024-08-23 22:15:00'),
(7232752187946734592, '/sys/roleapi/role_api_transfer_list', 'Get', '获取角色api所有的选择列表', '0', NULL, 8, '2024-08-23 22:15:00'),
(7232752188013843456, '/sys/roleapi/list', 'Get', '获取角色api列表', '0', NULL, 9, '2024-08-23 22:15:00'),
(7232752188085146624, '/sys/roleapi/edit', 'Put', '编辑角色api', '0', NULL, 10, '2024-08-23 22:15:00'),
(7232752188156449792, '/sys/roleapi/role_permission_list', 'Get', '根据角色id获取api权限列表', '0', NULL, 11, '2024-08-23 22:15:00'),
(7232752188219364352, '/sys/roleapi/add', 'Post', '添加角色api', '0', NULL, 12, '2024-08-23 22:15:00'),
(7232752188282278912, '/sys/dicttype/list', 'Get', '获取字典类型列表', '0', NULL, 13, '2024-08-23 22:15:00'),
(7232752188349387776, '/sys/dicttype/add', 'Post', '添加字典类型', '0', NULL, 14, '2024-08-23 22:15:00'),
(7232752188412302336, '/sys/dicttype/del', 'Delete', '删除字典类型', '0', NULL, 15, '2024-08-23 22:15:00'),
(7232752188479411200, '/sys/dicttype/edit', 'Put', '编辑字典类型', '0', NULL, 16, '2024-08-23 22:15:00'),
(7232752188546520064, '/sys/menu/edit', 'Put', '编辑菜单', '0', NULL, 17, '2024-08-23 22:15:00'),
(7232752188617823232, '/sys/menu/list', 'Get', '菜单列表', '0', NULL, 18, '2024-08-23 22:15:00'),
(7232752188697515008, '/sys/menu/add', 'Post', '添加菜单', '0', NULL, 19, '2024-08-23 22:15:00'),
(7232752188764623872, '/sys/menu/del', 'Delete', '删除菜单', '0', NULL, 20, '2024-08-23 22:15:00'),
(7232752188835927040, '/sys/menu/all_router', 'Get', '全部路由', '0', NULL, 21, '2024-08-23 22:15:00'),
(7232752188898841600, '/sys/menu/tree', 'Get', '获取菜单树', '0', NULL, 22, '2024-08-23 22:15:00'),
(7232752188965950464, '/sys/serverinfo/server_update', 'Get', '更新服务器信息', '0', NULL, 23, '2024-08-23 22:15:00'),
(7232752189028865024, '/sys/jobinfo/add', 'Post', '添加定时任务日志', '0', NULL, 24, '2024-08-23 22:15:00'),
(7232752189095973888, '/sys/jobinfo/list', 'Get', '获取定时任务日志列表', '0', NULL, 25, '2024-08-23 22:15:00'),
(7232752189158888448, '/sys/dictdata/get_by_type', 'Get', '根据类型获取字典数据', '0', NULL, 26, '2024-08-23 22:15:00'),
(7232752189221803008, '/sys/dictdata/list', 'Get', '获取字典数据列表', '0', NULL, 27, '2024-08-23 22:15:00'),
(7232752189280523264, '/sys/dictdata/del', 'Delete', '删除字典数据', '0', NULL, 28, '2024-08-23 22:15:00'),
(7232752189347632128, '/sys/dictdata/edit', 'Put', '编辑字典数据', '0', NULL, 29, '2024-08-23 22:15:00'),
(7232752189410546688, '/sys/dictdata/add', 'Post', '添加字典数据', '0', NULL, 30, '2024-08-23 22:15:00'),
(7232752189473461248, '/sys/dashboard/add', 'Post', '添加仪表盘', '0', NULL, 31, '2024-08-23 22:15:00'),
(7232752189557347328, '/sys/dashboard/list', 'Get', '获取仪表盘列表', '0', NULL, 32, '2024-08-23 22:15:00'),
(7232752189628650496, '/sys/role/list', 'Get', '获取角色列表', '0', NULL, 33, '2024-08-23 22:15:00'),
(7232752189691565056, '/sys/role/get_role_menus', 'Get', '获取角色菜单', '0', NULL, 34, '2024-08-23 22:15:00'),
(7232752189758673920, '/sys/role/menu', 'Get', '获取角色菜单', '0', NULL, 35, '2024-08-23 22:15:00'),
(7232752189821588480, '/sys/role/edit', 'Put', '编辑角色', '0', NULL, 36, '2024-08-23 22:15:00'),
(7232752189888697344, '/sys/role/tree', 'Get', '获取角色树', '0', NULL, 37, '2024-08-23 22:15:00'),
(7232752189951611904, '/sys/role/add', 'Post', '添加角色', '0', NULL, 38, '2024-08-23 22:15:00'),
(7232752190018720768, '/sys/role/del', 'Delete', '删除角色', '0', NULL, 39, '2024-08-23 22:15:00'),
(7232752190081635328, '/sys/apipermission/list', 'Get', '获取api权限列表', '0', NULL, 40, '2024-08-23 22:15:00'),
(7232752190144549888, '/sys/user/reset_password', 'Put', '重置密码', '0', NULL, 41, '2024-08-23 22:15:00'),
(7232752190207464448, '/sys/user/userinfo', 'Get', '获取用户信息', '0', NULL, 42, '2024-08-23 22:15:00'),
(7232752190270379008, '/sys/user/depts_roles', 'Get', '获取用户拥有的用户部门和用户角色', '0', NULL, 43, '2024-08-23 22:15:00'),
(7232752190333293568, '/sys/user/del', 'Delete', '删除用户', '0', NULL, 44, '2024-08-23 22:15:00'),
(7232752190396208128, '/sys/user/list', 'Get', '获取用户列表', '0', NULL, 45, '2024-08-23 22:15:00'),
(7232752190459122688, '/sys/user/add', 'Post', '添加用户', '0', NULL, 46, '2024-08-23 22:15:00'),
(7232752190526231552, '/sys/user/change_password', 'Put', '修改密码', '0', NULL, 47, '2024-08-23 22:15:00'),
(7232752190589146112, '/sys/user/edit', 'Put', '编辑用户', '0', NULL, 48, '2024-08-23 22:15:00'),
(7232752190656254976, '/sys/dept/edit', 'Put', '编辑部门', '0', NULL, 49, '2024-08-23 22:15:00'),
(7232752190727558144, '/sys/dept/add', 'Post', '添加部门', '0', NULL, 50, '2024-08-23 22:15:00'),
(7232752190794667008, '/sys/dept/list', 'Get', '获取部门列表', '0', NULL, 51, '2024-08-23 22:15:00'),
(7232752190861775872, '/sys/dept/del', 'Delete', '删除部门', '0', NULL, 52, '2024-08-23 22:15:00'),
(7232752190933079040, '/sys/dept/dept_tree', 'Get', '获取部门树', '0', NULL, 53, '2024-08-23 22:15:00'),
(7232752190995993600, '/sys/useronline/list', 'Get', '获取在线用户列表', '0', NULL, 54, '2024-08-23 22:15:00'),
(7232752191063102464, '/sys/logininfo/list', 'Get', '获取登录日志列表', '0', NULL, 55, '2024-08-23 22:15:00'),
(7232781502121939968, '/sys/user/delusers', 'Delete', '删除用户', '0', NULL, 25, '2024-08-24 00:11:29'),
(7232794958103483392, '/sys/dashboard/analysis/weeklyUserActivity', 'Get', '获取仪表盘数据', '0', NULL, 24, '2024-08-24 01:04:57'),
(7232794958170592256, '/sys/dashboard/analysis/total', 'Get', '获取仪表盘数据', '0', NULL, 25, '2024-08-24 01:04:57'),
(7232794958229312512, '/sys/dashboard/analysis/monthlySales', 'Get', '获取仪表盘数据', '0', NULL, 26, '2024-08-24 01:04:57'),
(7232794958279644160, '/sys/dashboard/analysis/userAccessSource', 'Get', '获取仪表盘数据', '0', NULL, 27, '2024-08-24 01:04:57'),
(7232794958317392896, '/sys/dashboard/workplace/dynamic', 'Get', '获取仪表盘数据', '0', NULL, 28, '2024-08-24 01:04:57'),
(7232794958363530240, '/sys/dashboard/workplace/total', 'Get', '获取仪表盘数据', '0', NULL, 29, '2024-08-24 01:04:57'),
(7232794958401278976, '/sys/dashboard/workplace/radar', 'Get', '获取仪表盘数据', '0', NULL, 30, '2024-08-24 01:04:57'),
(7232794958447416320, '/sys/dashboard/workplace/project', 'Get', '获取仪表盘数据', '0', NULL, 31, '2024-08-24 01:04:57'),
(7232794958493553664, '/sys/dashboard/workplace/team', 'Get', '获取仪表盘数据', '0', NULL, 32, '2024-08-24 01:04:57'),
(7233868069737501696, '/sys/apipermission/edit', 'Put', '编辑api权限', '0', NULL, 63, '2024-08-27 00:09:07'),
(7234253747373642752, '/sys/operationlog/list', 'Get', '获取操作日志列表', '0', NULL, 52, '2024-08-28 01:41:39'),
(7234570580194661376, '/sys/useronline/del', 'Delete', '退出在线用户', '0', NULL, 47, '2024-08-28 22:40:38'),
(7234616780964926464, '/sys/cache/clear', 'Post', '清空缓存', '0', NULL, 18, '2024-08-29 01:44:13'),
(7234616781027841024, '/sys/cache/list', 'Get', '获取缓存列表', '0', NULL, 19, '2024-08-29 01:44:13'),
(7235633673968456704, '/sys/role/get_role_depts', 'Get', '获取角色部门', '0', NULL, 12, '2024-08-31 21:04:59'),
(7235678367742071808, '/sys/job/hand_execute_job', 'Post', '执行定时任务', '0', NULL, 67, '2024-09-01 00:02:35'),
(7236784152022782976, '/sys/user/fresh_token', 'Put', '刷新token', '0', NULL, 36, '2024-09-04 01:16:35'),
(7236784153973134336, '/sys/user/update_avatar', 'Put', '更新头像', '0', NULL, 38, '2024-09-04 01:16:35'),
(7240020747752477696, '/sys/user/refersh_token', 'Put', '刷新token', '0', NULL, 66, '2024-09-12 23:37:39'),
(7241490807281062912, '/sys/user/login_out', 'Get', '用户退出', '0', NULL, 66, '2024-09-17 00:59:09'),
(7252729164858299392, '/sys/role/user_role_name_list', 'Get', '获取用户拥有的角色名称列表', '0', NULL, 14, '2024-10-18 01:16:22'),
(7252738573697192960, '/sys/dept/user_dept_name_list', 'Get', '获取用户拥有的部门名称列表', '0', NULL, 44, '2024-10-18 01:53:45'),
(7253067149336286208, '/sys/user/update_role_or_dept', 'Post', '更新用户拥有的用户部门和用户角色', '0', NULL, 57, '2024-10-18 23:39:24');

-- --------------------------------------------------------

--
-- 表的结构 `sys_dept`
--

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
  `deleted_at` datetime DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='部门表';

--
-- 转存表中的数据 `sys_dept`
--

INSERT INTO `sys_dept` (`dept_id`, `parent_id`, `dept_name`, `lft`, `rgt`, `depth`, `dept_category`, `order`, `leader`, `phone`, `email`, `status`, `remark`, `created_at`, `updated_at`, `deleted_at`) VALUES
(100, 0, '科技', 1, 26, 1, NULL, 1, NULL, '15888888888', 'xxx@qq.com', '0', NULL, '2024-07-03 01:17:31', NULL, NULL),
(101, 100, '深圳总公司', 2, 13, 2, NULL, 1, NULL, '15888888888', 'xxx@qq.com', '0', NULL, '2024-07-03 01:17:31', NULL, NULL),
(102, 100, '长沙分公司', 14, 21, 2, NULL, 2, NULL, '187554971', 'xxx@qq.com', '0', '长沙的公司', '2024-07-03 01:17:31', NULL, NULL),
(103, 101, '研发部门', 3, 4, 3, NULL, 1, 1, '15888888888', 'xxx@qq.com', '0', NULL, '2024-07-03 01:17:31', NULL, NULL),
(104, 101, '市场部门', 5, 6, 3, NULL, 2, NULL, '15888888888', 'xxx@qq.com', '0', NULL, '2024-07-03 01:17:31', NULL, NULL),
(105, 101, '测试部门', 7, 8, 3, NULL, 3, NULL, '15888888888', 'xxx@qq.com', '0', NULL, '2024-07-03 01:17:31', NULL, NULL),
(106, 101, '财务部门', 9, 10, 3, NULL, 4, NULL, '15888888888', 'xxx@qq.com', '0', NULL, '2024-07-03 01:17:31', NULL, NULL),
(107, 101, '运维部门', 11, 12, 3, NULL, 5, NULL, '15888888888', 'xxx@qq.com', '0', NULL, '2024-07-03 01:17:31', NULL, NULL),
(108, 102, '市场部门', 15, 16, 3, NULL, 1, NULL, '15888888888', 'xxx@qq.com', '0', NULL, '2024-07-03 01:17:31', NULL, NULL),
(109, 102, '财务部门', 17, 18, 3, NULL, 2, NULL, '15888888888', 'xxx@qq.com', '0', NULL, '2024-07-03 01:17:31', NULL, NULL),
(7223731313261547520, 102, '运维部门', 19, 20, 3, NULL, 3, NULL, NULL, NULL, '0', '长沙运维部门', NULL, NULL, NULL),
(7223731922878468096, 100, '广州分公司', 22, 25, 2, NULL, 5, NULL, NULL, NULL, '0', NULL, NULL, NULL, NULL),
(7223732104475054080, 7223731922878468096, '小部门', 23, 24, 3, NULL, 3, NULL, NULL, NULL, '0', NULL, NULL, NULL, NULL);

-- --------------------------------------------------------

--
-- 表的结构 `sys_dict_data`
--

CREATE TABLE `sys_dict_data` (
  `dict_code` bigint(20) NOT NULL COMMENT '字典编码',
  `dict_sort` int(4) NOT NULL DEFAULT '0' COMMENT '字典排序',
  `dict_label` varchar(100) NOT NULL DEFAULT '' COMMENT '字典标签',
  `dict_value` varchar(100) NOT NULL DEFAULT '' COMMENT '字典键值',
  `dict_type_id` bigint(20) NOT NULL COMMENT '字典类型',
  `created_at` datetime DEFAULT NULL COMMENT '创建时间',
  `updated_at` datetime DEFAULT NULL COMMENT '更新时间',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='字典数据表';

--
-- 转存表中的数据 `sys_dict_data`
--

INSERT INTO `sys_dict_data` (`dict_code`, `dict_sort`, `dict_label`, `dict_value`, `dict_type_id`, `created_at`, `updated_at`, `remark`) VALUES
(1, 1, '男', '0', 1, '2024-07-03 01:17:32', NULL, '性别男'),
(2, 2, '女', '1', 1, '2024-07-03 01:17:32', NULL, '性别女'),
(3, 3, '未知', '2', 1, '2024-07-03 01:17:32', NULL, '性别未知'),
(4, 1, '显示', '0', 2, '2024-07-03 01:17:32', NULL, '显示菜单a'),
(5, 2, '隐藏', '1', 2, '2024-07-03 01:17:32', NULL, '隐藏菜单'),
(6, 1, '正常', '0', 3, '2024-07-03 01:17:33', NULL, '正常状态'),
(7, 2, '停用', '1', 3, '2024-07-03 01:17:33', NULL, '停用状态'),
(12, 3, '是', 'Y', 6, '2024-07-03 01:17:33', NULL, '系统默认是'),
(13, 4, '否', 'N', 6, '2024-07-03 01:17:33', NULL, '系统默认否'),
(14, 1, '通知', '1', 7, '2024-07-03 01:17:33', NULL, '通知'),
(15, 2, '公告', '2', 7, '2024-07-03 01:17:33', NULL, '公告'),
(16, 1, '正常', '0', 8, '2024-07-03 01:17:33', NULL, '正常状态'),
(17, 2, '关闭', '1', 8, '2024-07-03 01:17:33', NULL, '关闭状态'),
(18, 1, '新增', '1', 9, '2024-07-03 01:17:33', NULL, '新增操作'),
(19, 2, '修改', '2', 9, '2024-07-03 01:17:33', NULL, '修改操作'),
(20, 3, '删除', '3', 9, '2024-07-03 01:17:33', NULL, '删除操作'),
(21, 4, '授权', '4', 9, '2024-07-03 01:17:33', NULL, '授权操作'),
(22, 5, '导出', '5', 9, '2024-07-03 01:17:33', NULL, '导出操作'),
(23, 6, '导入', '6', 9, '2024-07-03 01:17:33', NULL, '导入操作'),
(24, 7, '强退', '7', 9, '2024-07-03 01:17:33', NULL, '强退操作'),
(25, 8, '生成代码', '8', 9, '2024-07-03 01:17:33', NULL, '生成操作'),
(26, 9, '清空数据', '9', 9, '2024-07-03 01:17:33', NULL, '清空操作'),
(27, 1, '成功', '0', 10, '2024-07-03 01:17:33', NULL, '正常状态'),
(28, 2, '失败', '1', 10, '2024-07-03 01:17:33', NULL, '停用状态'),
(29, 99, '其他', '0', 9, '2024-07-03 01:17:33', NULL, '其他操作'),
(30, 0, '密码认证', 'password', 11, '2024-07-03 01:17:33', NULL, '密码认证'),
(31, 0, '短信认证', 'sms', 11, '2024-07-03 01:17:33', NULL, '短信认证'),
(32, 0, '邮件认证', 'email', 11, '2024-07-03 01:17:33', NULL, '邮件认证'),
(33, 0, '小程序认证', 'xcx', 11, '2024-07-03 01:17:33', NULL, '小程序认证'),
(34, 0, '三方登录认证', 'social', 11, '2024-07-03 01:17:33', NULL, '三方登录认证'),
(35, 0, 'PC', 'pc', 12, '2024-07-03 01:17:33', NULL, 'PC'),
(36, 0, '安卓', 'android', 12, '2024-07-03 01:17:33', NULL, '安卓'),
(37, 0, 'iOS', 'ios', 12, '2024-07-03 01:17:33', NULL, 'iOS'),
(38, 0, '小程序', 'xcx', 12, '2024-07-03 01:17:33', NULL, '小程序'),
(7229170738212769792, 1, '请求网址', 'geturl', 7229170480917385217, NULL, NULL, 'GET请求网址'),
(7229171315483217920, 3, '发送邮件', 'sendemail', 7229170480917385217, NULL, NULL, '发送邮件'),
(7230797167031619584, 1, '默认', 'default', 7230795611045498880, NULL, NULL, NULL),
(7230797303799484416, 3, '系统', 'system', 7230795611045498880, NULL, NULL, NULL),
(7233442254918947840, 1, '不记录', '0', 7233442155891430400, NULL, NULL, NULL),
(7233442310988403712, 3, '数据库', '2', 7233442155891430400, NULL, NULL, NULL),
(7233442350909789184, 2, '文件记录', '1', 7233442155891430400, NULL, NULL, NULL),
(7233442400423547904, 5, '数据库和文件', '3', 7233442155891430400, NULL, NULL, NULL),
(7235988025748919296, 3, '调用方法', 'invokefunction', 7229170480917385217, NULL, NULL, '调用方法');

-- --------------------------------------------------------

--
-- 表的结构 `sys_dict_type`
--

CREATE TABLE `sys_dict_type` (
  `dict_id` bigint(20) NOT NULL COMMENT '字典主键',
  `dict_name` varchar(100) NOT NULL COMMENT '字典名称',
  `dict_type` varchar(100) NOT NULL COMMENT '字典类型',
  `order` int(4) NOT NULL COMMENT '排序',
  `created_at` datetime NOT NULL COMMENT '创建时间',
  `updated_at` datetime NOT NULL COMMENT '更新时间',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='字典类型表';

--
-- 转存表中的数据 `sys_dict_type`
--

INSERT INTO `sys_dict_type` (`dict_id`, `dict_name`, `dict_type`, `order`, `created_at`, `updated_at`, `remark`) VALUES
(1, '用户性别', 'sys_user_sex', 103, '2024-07-03 01:17:32', '2024-07-03 01:17:32', '用户性别列表'),
(2, '菜单状态', 'sys_show_hide', 103, '2024-07-03 01:17:32', '2024-07-03 01:17:32', '菜单状态列表'),
(3, '系统开关', 'sys_normal_disable', 103, '2024-07-03 01:17:32', '2024-07-03 01:17:32', '系统开关列表'),
(6, '系统是否', 'sys_yes_no', 103, '2024-07-03 01:17:32', '2024-07-03 01:17:32', '系统是否列表'),
(7, '通知类型', 'sys_notice_type', 103, '2024-07-03 01:17:32', '2024-08-14 01:06:32', '通知类型列表'),
(8, '通知状态', 'sys_notice_status', 103, '2024-07-03 01:17:32', '2024-07-03 01:17:32', '通知状态列表'),
(9, '操作类型', 'sys_oper_type', 103, '2024-07-03 01:17:32', '2024-07-03 01:17:32', '操作类型列表'),
(10, '系统状态', 'sys_common_status', 103, '2024-07-03 01:17:32', '2024-07-03 01:17:32', '登录状态列表'),
(11, '授权类型', 'sys_grant_type', 103, '2024-07-03 01:17:32', '2024-07-03 01:17:32', '认证授权类型'),
(12, '设备类型', 'sys_device_type', 103, '2024-07-03 01:17:32', '2024-07-03 01:17:32', '客户端设备类型'),
(7229170480917385217, '定时任务', 'sys_job', 1, '2024-08-14 01:02:34', '2024-08-14 01:02:34', '定时任务类型'),
(7230795611045498880, '任务分组', 'sys_job_group', 3, '2024-08-18 12:40:15', '2024-08-18 12:40:15', '定时任务分组'),
(7233442155891430400, '日志记录', 'sys_logcache', 2, '2024-08-25 19:56:41', '2024-08-25 20:46:41', '日志记录'),
(7235877375018177536, '数据权限', 'sys_data_scope', 4, '2024-09-01 13:13:22', '2024-09-01 13:13:22', '数据权限');

-- --------------------------------------------------------

--
-- 表的结构 `sys_job`
--

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
  `remark` varchar(512) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

--
-- 转存表中的数据 `sys_job`
--

INSERT INTO `sys_job` (`created_at`, `updated_at`, `job_id`, `task_type`, `task_count`, `run_count`, `job_name`, `job_params`, `job_group`, `cron_expression`, `status`, `remark`) VALUES
('2024-08-16 00:34:21', '2024-08-29 22:01:40', 7227378358836924416, 'geturl', 0, 1737, '请求百度', 'https://www.baidu.com/', 'default', '30 * * * * ?', '1', '66'),
('2024-08-16 00:34:21', '2024-08-29 22:01:44', 7227378358836924417, 'geturl', 0, 3479, '请求QQ', 'https://www.qq.com/', 'default', '0/30 * * * * ?', '1', '99'),
('2024-09-01 12:35:38', '2024-09-01 22:24:37', 7235988676738454528, 'invokefunction', 0, 13, '清理过期的Token', '{\"callfun\":\"clearuserinfo\"}', 'default', '* * 2 * * ?', '1', '晚上两点执行清理过期的Token');

-- --------------------------------------------------------

--
-- 表的结构 `sys_job_log`
--

CREATE TABLE `sys_job_log` (
  `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `id` bigint(20) NOT NULL,
  `job_id` bigint(20) NOT NULL,
  `run_count` int(11) NOT NULL,
  `job_message` varchar(2048) DEFAULT NULL,
  `status` char(32) NOT NULL,
  `elapsed_time` datetime DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- --------------------------------------------------------

--
-- 表的结构 `sys_login_info`
--

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
  `login_time` datetime DEFAULT NULL COMMENT '访问时间'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='系统访问记录';

-- --------------------------------------------------------

--
-- 表的结构 `sys_menu`
--

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
  `deleted_at` datetime DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='菜单权限表';

--
-- 转存表中的数据 `sys_menu`
--

INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_at`, `remark`, `deleted_at`) VALUES
(1, 'Dashboard', '首页', NULL, 0, 0, '/dashboard', '#', '/dashboard/analysis', '', '0', 'M', '0', '0', '0', '1', '0', '1', '1', '0', NULL, 'vi-ant-design:dashboard-filled', NULL, NULL, '', NULL),
(2, 'Analysis', '分析页', NULL, 1, 1, 'analysis', 'views/Dashboard/Analysis', NULL, '', '0', 'C', '0', '0', '0', '1', '0', '1', '1', '0', NULL, 'vi-ant-design:dashboard-filled', NULL, '2024-08-23 22:51:09', NULL, NULL),
(3, 'Workplace', '工作台', NULL, 1, 2, 'workplace', 'views/Dashboard/Workplace', NULL, '', '0', 'C', '0', '0', '0', '1', '0', '1', '1', '0', NULL, 'vi-ant-design:codepen-outlined', NULL, '2024-08-23 22:50:07', NULL, NULL),
(4, 'Authorization', '系统管理', NULL, 0, 2, '/authorization', '#', NULL, '', '0', 'M', '0', '0', '0', '1', '0', '1', '1', '0', NULL, 'vi-ant-design:dashboard-filled', NULL, NULL, '', NULL),
(5, 'Department', '部门管理', NULL, 4, 5, 'department', 'views/Authorization/Department/Department', NULL, '', '0', 'C', '0', '0', '0', '1', '0', '1', '1', '0', NULL, 'vi-ant-design:dashboard-filled', NULL, '2024-08-11 05:31:40', NULL, NULL),
(6, 'User', '用户管理', NULL, 4, 2, 'user', 'views/Authorization/User/User', NULL, '', '0', 'C', '0', '0', '0', '1', '0', '1', '1', '0', NULL, 'vi-ant-design:dashboard-filled', NULL, NULL, '', NULL),
(7, 'Menu', '菜单管理', NULL, 4, 2, 'menu', 'views/Authorization/Menu/Menu', NULL, '', '0', 'C', '0', '0', '0', '1', '0', '0', '1', '0', NULL, 'vi-ant-design:dashboard-filled', NULL, '2024-08-08 00:50:11', NULL, NULL),
(8, 'Role', '角色管理', NULL, 4, 2, 'role', 'views/Authorization/Role/Role', NULL, '', '0', 'C', '0', '0', '0', '1', '0', '1', '1', '0', NULL, 'vi-ant-design:dashboard-filled', NULL, NULL, '', NULL),
(7224471859710005249, 'Monitor', '系统监控', NULL, 0, 5, '/motion', '#', NULL, NULL, '0', 'M', '0', '0', '0', '1', '0', '1', '1', '0', NULL, 'vi-tdesign:archway', NULL, '2024-10-25 00:42:23', NULL, NULL),
(7225437966700777473, 'SystemLog', '操作日志', NULL, 7224471859710005249, 4, 'systemlog', 'views/Monitor/SystemLog/SystemLog', NULL, NULL, '0', 'C', '0', '0', '0', '1', '0', '0', '0', '0', NULL, 'vi-tdesign:angry', NULL, '2024-08-28 01:49:53', NULL, NULL),
(7225439414641627136, 'LoginInfo', '登陆日志', NULL, 7224471859710005249, 2, 'logininfo', 'views/Monitor/LoginInfo/LoginInfo', NULL, NULL, '0', 'C', '0', '0', '0', '1', '0', '0', '0', '0', NULL, 'vi-tdesign:app', NULL, '2024-08-25 00:53:36', NULL, NULL),
(7226994818660044800, 'UserOnline', '在线用户', NULL, 7224471859710005249, 0, 'useronline', 'views/Monitor/UserOnline/UserOnline', NULL, '', '0', 'C', '0', '0', '0', '1', '0', '1', '1', '0', NULL, 'vi-ep:bell-filled', '2024-08-08 00:57:16', '2024-10-24 01:28:11', NULL, NULL),
(7227715297679118337, 'Job', '定时任务', NULL, 7224471859710005249, 0, 'job', 'views/Monitor/Job/Job', NULL, '', '0', 'C', '0', '0', '0', '0', '0', '0', '0', '0', NULL, 'vi-ep:apple', '2024-08-10 00:40:11', '2024-08-10 00:40:11', NULL, NULL),
(7228150987080470529, 'DictData', '字典数据', NULL, 4, 100, 'dictdata', 'views/Authorization/DictData/DictData', NULL, '', '0', 'C', '1', '0', '0', '0', '0', '1', '0', '0', NULL, 'vi-ant-design:aliwangwang-outlined', '2024-08-11 05:31:28', '2024-10-25 23:18:20', NULL, NULL),
(7228186533035118593, 'DictType', '字典管理', NULL, 4, 7, 'dicttype', 'views/Authorization/DictType/DictType', NULL, '', '0', 'C', '0', '0', '0', '0', '0', '0', '0', '0', NULL, 'vi-ant-design:alert-filled', '2024-08-11 07:52:43', '2024-10-25 23:18:11', NULL, NULL),
(7228454585970593792, 'ServerInfo', '服务监控', NULL, 7224471859710005249, 4, 'serverinfo', 'views/Monitor/ServerInfo/ServerInfo', NULL, '', '1', 'C', '0', '0', '0', '0', '0', '0', '0', '0', NULL, 'vi-ant-design:fund-projection-screen-outlined', '2024-08-12 01:37:51', '2024-08-25 00:54:16', NULL, NULL),
(7230621134038896640, 'JobLog', '定时日志', NULL, 7224471859710005249, 0, 'joblog', 'views/Monitor/JobLog/JobLog', NULL, '', '1', 'C', '1', '0', '0', '0', '0', '0', '0', '0', NULL, 'vi-ep:baseball', '2024-08-18 01:06:57', '2024-09-01 00:09:39', NULL, NULL),
(7230940389695426560, 'ApiPermission', '权限查询', NULL, 4, 7, 'apipermission', 'views/Authorization/ApiPermission/ApiPermission', NULL, '', '0', 'C', '0', '0', '0', '0', '0', '0', '0', '0', NULL, 'vi-ep:burger', '2024-08-18 22:15:33', '2024-08-24 01:22:22', NULL, NULL),
(7231673632416502784, 'RoleApi', '角色API', NULL, 4, 1, 'roleapi', 'views/Authorization/RoleApi/RoleApi', NULL, '', '0', 'C', '1', '0', '0', '0', '0', '0', '0', '0', NULL, 'vi-ep:basketball', '2024-08-20 22:49:12', '2024-10-24 01:32:15', NULL, NULL),
(7234621748841190400, 'Cache', '缓存管理', NULL, 7224471859710005249, 1, 'cache', 'views/Monitor/Cache/Cache', NULL, '', '0', 'C', '0', '0', '0', '0', '0', '0', '0', '0', NULL, 'vi-ep:bicycle', '2024-08-29 02:03:58', '2024-08-29 02:03:58', NULL, NULL);

-- --------------------------------------------------------

--
-- 表的结构 `sys_notice`
--

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
  `remark` varchar(255) DEFAULT NULL COMMENT '备注'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='通知公告表';

--
-- 转存表中的数据 `sys_notice`
--

INSERT INTO `sys_notice` (`notice_id`, `notice_title`, `notice_type`, `notice_content`, `status`, `create_dept`, `create_by`, `created_at`, `update_by`, `updated_at`, `remark`) VALUES
(1, '温馨提醒：2018-07-01 新版本发布啦', '2', 0xe696b0e78988e69cace58685e5aeb9, '0', 103, 1, '2024-07-03 01:17:33', NULL, NULL, '管理员'),
(2, '维护通知：2018-07-01 系统凌晨维护', '1', 0xe7bbb4e68aa4e58685e5aeb9, '0', 103, 1, '2024-07-03 01:17:33', NULL, NULL, '管理员');

-- --------------------------------------------------------

--
-- 表的结构 `sys_oper_log`
--

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
  `cost_time` bigint(20) DEFAULT '0' COMMENT '消耗时间'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='操作日志记录';

-- --------------------------------------------------------

--
-- 表的结构 `sys_post`
--

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
  `remark` varchar(500) DEFAULT NULL COMMENT '备注'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='岗位信息表';

--
-- 转存表中的数据 `sys_post`
--

INSERT INTO `sys_post` (`post_id`, `dept_id`, `post_code`, `post_category`, `post_name`, `post_sort`, `status`, `create_dept`, `created_at`, `updated_at`, `remark`) VALUES
(1, 103, 'ceo', NULL, '董事长', 1, '0', 103, '2024-07-03 01:17:32', NULL, ''),
(2, 100, 'se', NULL, '项目经理', 2, '0', 103, '2024-07-03 01:17:32', NULL, ''),
(3, 100, 'hr', NULL, '人力资源', 3, '0', 103, '2024-07-03 01:17:32', NULL, ''),
(4, 100, 'user', NULL, '普通员工', 4, '0', 103, '2024-07-03 01:17:32', NULL, '');

-- --------------------------------------------------------

--
-- 表的结构 `sys_role`
--

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
  `deleted_at` datetime DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='角色信息表';

--
-- 转存表中的数据 `sys_role`
--

INSERT INTO `sys_role` (`role_id`, `role_name`, `role_key`, `order`, `data_scope`, `status`, `create_dept`, `remark`, `created_at`, `updated_at`, `deleted_at`) VALUES
(1, '管理员', 'admin', 2, '1', '0', 103, '超级管理员', '2024-07-03 01:17:32', NULL, NULL),
(3, '本部门及以下', 'test1', 5, '4', '0', 103, '本部门数据', '2024-07-03 01:17:32', NULL, NULL),
(4, '仅本人', 'test2', 6, '5', '0', 103, '本人数据', '2024-07-03 01:17:32', NULL, NULL),
(7222985345490620417, '自己权限', 'self', 7, '2', '0', NULL, 'ffad', NULL, NULL, NULL);

-- --------------------------------------------------------

--
-- 表的结构 `sys_role_api`
--

CREATE TABLE `sys_role_api` (
  `id` bigint(20) NOT NULL,
  `role_id` bigint(20) NOT NULL,
  `api_id` bigint(20) NOT NULL,
  `api` char(255) NOT NULL,
  `method` char(10) NOT NULL,
  `apiname` char(32) NOT NULL,
  `sort` int(11) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

--
-- 转存表中的数据 `sys_role_api`
--

INSERT INTO `sys_role_api` (`id`, `role_id`, `api_id`, `api`, `method`, `apiname`, `sort`) VALUES
(7252743290347230208, 3, 7232752187397280768, '/sys/job/add', 'Post', '添加定时任务', 1),
(7252743290611471360, 3, 7232752187535692800, '/sys/job/validate_cron', 'Post', '验证cron表达式', 2),
(7252743290825380864, 3, 7232752187606995968, '/sys/job/list', 'Get', '获取定时任务列表', 3),
(7252743291056067584, 3, 7232752187674104832, '/sys/job/edit', 'Put', '编辑定时任务', 4),
(7252743291269977088, 3, 7232752187741213696, '/sys/job/del', 'Delete', '删除定时任务', 5),
(7252743291479692288, 3, 7232752187812516864, '/sys/roleapi/add_many_role_api_transfer', 'Get', '添加多个角色api', 6),
(7252743291693601792, 3, 7232752187879625728, '/sys/roleapi/del', 'Delete', '删除角色api', 7),
(7252743291907511296, 3, 7232752187946734592, '/sys/roleapi/role_api_transfer_list', 'Get', '获取角色api所有的选择列表', 8),
(7252743292117226496, 3, 7232752188013843456, '/sys/roleapi/list', 'Get', '获取角色api列表', 9),
(7252743292339524608, 3, 7232752188085146624, '/sys/roleapi/edit', 'Put', '编辑角色api', 10),
(7252743292540851200, 3, 7232752188156449792, '/sys/roleapi/role_permission_list', 'Get', '根据角色id获取api权限列表', 11),
(7252743292737983488, 3, 7232752188219364352, '/sys/roleapi/add', 'Post', '添加角色api', 12),
(7252743292947698688, 3, 7232752188282278912, '/sys/dicttype/list', 'Get', '获取字典类型列表', 13),
(7252743293161608192, 3, 7252729164858299392, '/sys/role/user_role_name_list', 'Get', '获取用户拥有的角色名称列表', 14),
(7252743293379712000, 3, 7232752188349387776, '/sys/dicttype/add', 'Post', '添加字典类型', 15),
(7252743293585232896, 3, 7232752188412302336, '/sys/dicttype/del', 'Delete', '删除字典类型', 16),
(7252743293794948096, 3, 7232752188479411200, '/sys/dicttype/edit', 'Put', '编辑字典类型', 17),
(7252743294008857600, 3, 7232752188546520064, '/sys/menu/edit', 'Put', '编辑菜单', 18),
(7252743294218572800, 3, 7232752188617823232, '/sys/menu/list', 'Get', '菜单列表', 19),
(7252743294428288000, 3, 7232752188697515008, '/sys/menu/add', 'Post', '添加菜单', 20),
(7252743294646391808, 3, 7232752188764623872, '/sys/menu/del', 'Delete', '删除菜单', 21),
(7252743294851912704, 3, 7232752188835927040, '/sys/menu/all_router', 'Get', '全部路由', 22),
(7252743295065822208, 3, 7232752188898841600, '/sys/menu/tree', 'Get', '获取菜单树', 23),
(7252743295275537408, 3, 7232752188965950464, '/sys/serverinfo/server_update', 'Get', '更新服务器信息', 24),
(7252743295489446912, 3, 7232752189028865024, '/sys/jobinfo/add', 'Post', '添加定时任务日志', 25),
(7252743295728522240, 3, 7232794958103483392, '/sys/dashboard/analysis/weeklyUserActivity', 'Get', '获取仪表盘数据', 26),
(7252743295934043136, 3, 7232752189095973888, '/sys/jobinfo/list', 'Get', '获取定时任务日志列表', 27),
(7252743296143758336, 3, 7232794958170592256, '/sys/dashboard/analysis/total', 'Get', '获取仪表盘数据', 28),
(7252743296349279232, 3, 7232752189158888448, '/sys/dictdata/get_by_type', 'Get', '根据类型获取字典数据', 29),
(7252743296567383040, 3, 7232794958229312512, '/sys/dashboard/analysis/monthlySales', 'Get', '获取仪表盘数据', 30),
(7252743296768709632, 3, 7232794958279644160, '/sys/dashboard/analysis/userAccessSource', 'Get', '获取仪表盘数据', 31),
(7252743296974230528, 3, 7232752189221803008, '/sys/dictdata/list', 'Get', '获取字典数据列表', 32),
(7252743297179751424, 3, 7232752189280523264, '/sys/dictdata/del', 'Delete', '删除字典数据', 33),
(7252743297393660928, 3, 7232794958317392896, '/sys/dashboard/workplace/dynamic', 'Get', '获取仪表盘数据', 34),
(7252743297607570432, 3, 7232794958363530240, '/sys/dashboard/workplace/total', 'Get', '获取仪表盘数据', 35),
(7252743297821479936, 3, 7232752189347632128, '/sys/dictdata/edit', 'Put', '编辑字典数据', 36),
(7252743298031195136, 3, 7232752189410546688, '/sys/dictdata/add', 'Post', '添加字典数据', 37),
(7252743298253493248, 3, 7232794958401278976, '/sys/dashboard/workplace/radar', 'Get', '获取仪表盘数据', 38),
(7252743298467402752, 3, 7232794958447416320, '/sys/dashboard/workplace/project', 'Get', '获取仪表盘数据', 39),
(7252743298693895168, 3, 7232752189473461248, '/sys/dashboard/add', 'Post', '添加仪表盘', 40),
(7252743298907804672, 3, 7232794958493553664, '/sys/dashboard/workplace/team', 'Get', '获取仪表盘数据', 41),
(7252743299117519872, 3, 7232752189557347328, '/sys/dashboard/list', 'Get', '获取仪表盘列表', 42),
(7252743299331429376, 3, 7232752189628650496, '/sys/role/list', 'Get', '获取角色列表', 43),
(7252743299541144576, 3, 7232752189691565056, '/sys/role/get_role_menus', 'Get', '获取角色菜单', 44),
(7252743299750859776, 3, 7232752189758673920, '/sys/role/menu', 'Get', '获取角色菜单', 45),
(7252743299960574976, 3, 7232752189821588480, '/sys/role/edit', 'Put', '编辑角色', 46),
(7252743300174484480, 3, 7232752189888697344, '/sys/role/tree', 'Get', '获取角色树', 47),
(7252743300388393984, 3, 7232752189951611904, '/sys/role/add', 'Post', '添加角色', 48),
(7252743300589720576, 3, 7232752190018720768, '/sys/role/del', 'Delete', '删除角色', 49),
(7252743300803630080, 3, 7232752190081635328, '/sys/apipermission/list', 'Get', '获取api权限列表', 50),
(7252743301013345280, 3, 7232752190144549888, '/sys/user/reset_password', 'Put', '重置密码', 51),
(7252743301231449088, 3, 7232752190207464448, '/sys/user/userinfo', 'Get', '获取用户信息', 52),
(7252743301449552896, 3, 7232752190270379008, '/sys/user/depts_roles', 'Get', '获取用户拥有的用户部门和用户角色', 53),
(7252743301659268096, 3, 7232752190333293568, '/sys/user/del', 'Delete', '删除用户', 54),
(7252743301873177600, 3, 7252738573697192960, '/sys/dept/user_dept_name_list', 'Get', '获取用户拥有的部门名称列表', 55),
(7252743302087087104, 3, 7232752190396208128, '/sys/user/list', 'Get', '获取用户列表', 56),
(7252743302309385216, 3, 7232752190459122688, '/sys/user/add', 'Post', '添加用户', 57),
(7252743302527489024, 3, 7232752190526231552, '/sys/user/change_password', 'Put', '修改密码', 58),
(7252743302733009920, 3, 7232752190589146112, '/sys/user/edit', 'Put', '编辑用户', 59),
(7252743302942725120, 3, 7232752190656254976, '/sys/dept/edit', 'Put', '编辑部门', 60),
(7252743303152440320, 3, 7232752190727558144, '/sys/dept/add', 'Post', '添加部门', 61),
(7252743303366349824, 3, 7232752190794667008, '/sys/dept/list', 'Get', '获取部门列表', 62),
(7252743303592842240, 3, 7232752190861775872, '/sys/dept/del', 'Delete', '删除部门', 63),
(7252743303798363136, 3, 7232752190933079040, '/sys/dept/dept_tree', 'Get', '获取部门树', 64),
(7252743304008078336, 3, 7232752190995993600, '/sys/useronline/list', 'Get', '获取在线用户列表', 65),
(7252743304230376448, 3, 7232752191063102464, '/sys/logininfo/list', 'Get', '获取登录日志列表', 66);

-- --------------------------------------------------------

--
-- 表的结构 `sys_role_dept`
--

CREATE TABLE `sys_role_dept` (
  `role_id` bigint(20) NOT NULL COMMENT '角色ID',
  `dept_id` bigint(20) NOT NULL COMMENT '部门ID'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='角色和部门关联表';

--
-- 转存表中的数据 `sys_role_dept`
--

INSERT INTO `sys_role_dept` (`role_id`, `dept_id`) VALUES
(7222985345490620417, 101),
(7222985345490620417, 103),
(7222985345490620417, 104);

-- --------------------------------------------------------

--
-- 表的结构 `sys_role_menu`
--

CREATE TABLE `sys_role_menu` (
  `role_id` bigint(20) NOT NULL COMMENT '角色ID',
  `menu_id` bigint(20) NOT NULL COMMENT '菜单ID'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='角色和菜单关联表';

--
-- 转存表中的数据 `sys_role_menu`
--

INSERT INTO `sys_role_menu` (`role_id`, `menu_id`) VALUES
(1, 1),
(3, 1),
(7222985345490620417, 1),
(1, 2),
(3, 2),
(4, 2),
(7222985345490620417, 2),
(1, 3),
(7222985345490620417, 3),
(1, 4),
(3, 4),
(7222985345490620417, 4),
(1, 5),
(3, 5),
(7222985345490620417, 5),
(1, 6),
(3, 6),
(4, 6),
(7222985345490620417, 6),
(1, 7),
(4, 7),
(7222985345490620417, 7),
(1, 8),
(4, 8),
(7222985345490620417, 8),
(3, 7224471859710005249),
(3, 7225439414641627136),
(3, 7228454585970593792);

-- --------------------------------------------------------

--
-- 表的结构 `sys_user`
--

CREATE TABLE `sys_user` (
  `id` bigint(20) NOT NULL COMMENT '用户ID',
  `tenant_id` varchar(20) NOT NULL DEFAULT '000000' COMMENT '租户编号',
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
  `remark` varchar(500) DEFAULT NULL COMMENT '备注'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户信息表';

--
-- 转存表中的数据 `sys_user`
--

INSERT INTO `sys_user` (`id`, `tenant_id`, `dept_id`, `role_id`, `user_name`, `nick_name`, `user_type`, `email`, `phonenumber`, `sex`, `avatar`, `password`, `status`, `login_ip`, `login_date`, `create_dept`, `create_by`, `created_at`, `update_by`, `updated_at`, `deleted_at`, `remark`) VALUES
(1, '000000', 100, 1, 'admin', '管理员', 'sys_user', 'willmagic@qq.com', '18588719996', '1', 'static/2024-10/25_7255598825434158080.png', '$argon2id$v=19$m=19456,t=2,p=1$M6+FVFGkM/zIAC7VZgGoLQ$zSAK9IIFN28ZsskO5s1eKKNv2g4yGw2b+CKjRa1cLvo', '0', '127.0.0.1', '2024-07-03 01:17:31', 103, 1, '2024-07-03 01:17:31', NULL, NULL, NULL, '管理员'),
(3, '000000', 104, 3, 'test', '测试1', 'sys_user', 'test@qq.com', '15888888889', '0', NULL, '$argon2id$v=19$m=19456,t=2,p=1$mO97IyGb5zSmoGrmB3ET1A$x8CKk5Z33siO3zVtMUm2AjMQsRrfNISa35S+sGmksf4', '0', '127.0.0.1', '2024-07-03 01:17:31', 103, 1, '2024-07-03 01:17:31', 3, '2024-07-03 01:17:31', NULL, NULL),
(4, '000000', 102, 3, 'test1', '测试q', 'sys_user', 'test1@qq.com', '15888888887', '0', NULL, '$2a$10$b8yUzN0C71sbz.PhNOCgJe.Tu1yWC3RNrTyjSQ8p1W0.aaUXUJ.Ne', '0', '127.0.0.1', '2024-07-03 01:17:31', 103, 1, '2024-07-03 01:17:31', 4, '2024-07-03 01:17:31', '2024-08-24 00:21:07', NULL),
(5, '000000', 102, 3, 'test2', '测试2', 'sys_user', 'test2@qq.com', '15888888886', '0', NULL, '$2a$10$b8yUzN0C71sbz.PhNOCgJe.Tu1yWC3RNrTyjSQ8p1W0.aaUXUJ.Ne', '0', '127.0.0.1', '2024-07-03 01:17:31', 103, 1, '2024-07-03 01:17:31', 4, '2024-07-03 01:17:31', NULL, NULL),
(7222295451336314881, '000000', 103, 4, 'test3', '测试3', 'sys_user', 'test3@qq.com', '0588445337', '1', NULL, '', '0', '', NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL),
(7222299781468655617, '000000', 104, 3, 'test4', '测试4', 'sys_user', 'test4@qq.com', '0588445337', '0', NULL, '$argon2id$v=19$m=19456,t=2,p=1$JJh+GICj7Ii7dnexWgNrZQ$h/MU+GhTX5O9SMu7p9S899QUBWLM8QPo3lpFk7KGars', '0', '', NULL, NULL, NULL, NULL, NULL, NULL, NULL, '1'),
(7223731680347033600, '000000', 7223731313261547520, 7222985345490620417, 'test5', 'test5', 'sys_user', 'test5@qq.com', '0588445337', NULL, NULL, '$argon2id$v=19$m=19456,t=2,p=1$VsofXU4ao39GasijfbQvjw$sASrh2idw664BmrRN8XPguppliyXbFmYJhvclNtSi3w', NULL, '', NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL);

-- --------------------------------------------------------

--
-- 表的结构 `sys_user_dept`
--

CREATE TABLE `sys_user_dept` (
  `user_id` bigint(20) NOT NULL,
  `dept_id` bigint(20) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

--
-- 转存表中的数据 `sys_user_dept`
--

INSERT INTO `sys_user_dept` (`user_id`, `dept_id`) VALUES
(1, 100),
(1, 103),
(7222295451336314881, 103),
(3, 104),
(7222299781468655617, 104),
(7223731680347033600, 7223731313261547520);

-- --------------------------------------------------------

--
-- 表的结构 `sys_user_post`
--

CREATE TABLE `sys_user_post` (
  `user_id` bigint(20) NOT NULL COMMENT '用户ID',
  `post_id` bigint(20) NOT NULL COMMENT '岗位ID'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户与岗位关联表';

--
-- 转存表中的数据 `sys_user_post`
--

INSERT INTO `sys_user_post` (`user_id`, `post_id`) VALUES
(1, 1);

-- --------------------------------------------------------

--
-- 表的结构 `sys_user_role`
--

CREATE TABLE `sys_user_role` (
  `user_id` bigint(20) NOT NULL COMMENT '用户ID',
  `role_id` bigint(20) NOT NULL COMMENT '角色ID'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户和角色关联表';

--
-- 转存表中的数据 `sys_user_role`
--

INSERT INTO `sys_user_role` (`user_id`, `role_id`) VALUES
(1, 1),
(1, 3),
(3, 3),
(7222299781468655617, 3),
(4, 4),
(7222295451336314881, 4),
(7223731680347033600, 7222985345490620417);

-- --------------------------------------------------------

--
-- 表的结构 `sys_white_jwt`
--

CREATE TABLE `sys_white_jwt` (
  `jid` bigint(20) NOT NULL,
  `uid` bigint(20) NOT NULL,
  `token_id` bigint(20) NOT NULL,
  `info_id` bigint(20) NOT NULL,
  `token_expr` bigint(20) NOT NULL,
  `created_at` datetime NOT NULL,
  `update_at` datetime NOT NULL,
  `deleted_at` datetime DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='jwt白名单';

--
-- 转储表的索引
--

--
-- 表的索引 `seaql_migrations`
--
ALTER TABLE `seaql_migrations`
  ADD PRIMARY KEY (`version`);

--
-- 表的索引 `sys_api_permission`
--
ALTER TABLE `sys_api_permission`
  ADD PRIMARY KEY (`id`),
  ADD UNIQUE KEY `api` (`api`);

--
-- 表的索引 `sys_dept`
--
ALTER TABLE `sys_dept`
  ADD PRIMARY KEY (`dept_id`);

--
-- 表的索引 `sys_dict_data`
--
ALTER TABLE `sys_dict_data`
  ADD PRIMARY KEY (`dict_code`);

--
-- 表的索引 `sys_dict_type`
--
ALTER TABLE `sys_dict_type`
  ADD PRIMARY KEY (`dict_id`),
  ADD UNIQUE KEY `tenant_id` (`dict_type`);

--
-- 表的索引 `sys_job`
--
ALTER TABLE `sys_job`
  ADD PRIMARY KEY (`job_id`),
  ADD UNIQUE KEY `job_id` (`job_id`);

--
-- 表的索引 `sys_job_log`
--
ALTER TABLE `sys_job_log`
  ADD PRIMARY KEY (`id`),
  ADD UNIQUE KEY `id` (`id`);

--
-- 表的索引 `sys_login_info`
--
ALTER TABLE `sys_login_info`
  ADD PRIMARY KEY (`info_id`),
  ADD KEY `idx_sys_logininfor_s` (`status`),
  ADD KEY `idx_sys_logininfor_lt` (`login_time`);

--
-- 表的索引 `sys_menu`
--
ALTER TABLE `sys_menu`
  ADD PRIMARY KEY (`id`);

--
-- 表的索引 `sys_notice`
--
ALTER TABLE `sys_notice`
  ADD PRIMARY KEY (`notice_id`);

--
-- 表的索引 `sys_oper_log`
--
ALTER TABLE `sys_oper_log`
  ADD PRIMARY KEY (`oper_id`),
  ADD KEY `idx_sys_oper_log_s` (`status`),
  ADD KEY `idx_sys_oper_log_ot` (`oper_time`);

--
-- 表的索引 `sys_post`
--
ALTER TABLE `sys_post`
  ADD PRIMARY KEY (`post_id`);

--
-- 表的索引 `sys_role`
--
ALTER TABLE `sys_role`
  ADD PRIMARY KEY (`role_id`);

--
-- 表的索引 `sys_role_api`
--
ALTER TABLE `sys_role_api`
  ADD PRIMARY KEY (`id`);

--
-- 表的索引 `sys_role_dept`
--
ALTER TABLE `sys_role_dept`
  ADD PRIMARY KEY (`role_id`,`dept_id`),
  ADD KEY `dept_id` (`dept_id`);

--
-- 表的索引 `sys_role_menu`
--
ALTER TABLE `sys_role_menu`
  ADD PRIMARY KEY (`role_id`,`menu_id`),
  ADD KEY `menu_id` (`menu_id`);

--
-- 表的索引 `sys_user`
--
ALTER TABLE `sys_user`
  ADD PRIMARY KEY (`id`);

--
-- 表的索引 `sys_user_dept`
--
ALTER TABLE `sys_user_dept`
  ADD PRIMARY KEY (`user_id`,`dept_id`),
  ADD KEY `dept_id` (`dept_id`);

--
-- 表的索引 `sys_user_post`
--
ALTER TABLE `sys_user_post`
  ADD PRIMARY KEY (`user_id`,`post_id`),
  ADD KEY `post_id` (`post_id`);

--
-- 表的索引 `sys_user_role`
--
ALTER TABLE `sys_user_role`
  ADD PRIMARY KEY (`user_id`,`role_id`),
  ADD KEY `role_id` (`role_id`);

--
-- 表的索引 `sys_white_jwt`
--
ALTER TABLE `sys_white_jwt`
  ADD PRIMARY KEY (`jid`);

--
-- 限制导出的表
--

--
-- 限制表 `sys_role_dept`
--
ALTER TABLE `sys_role_dept`
  ADD CONSTRAINT `sys_role_dept_ibfk_1` FOREIGN KEY (`role_id`) REFERENCES `sys_role` (`role_id`),
  ADD CONSTRAINT `sys_role_dept_ibfk_2` FOREIGN KEY (`dept_id`) REFERENCES `sys_dept` (`dept_id`);

--
-- 限制表 `sys_role_menu`
--
ALTER TABLE `sys_role_menu`
  ADD CONSTRAINT `sys_role_menu_ibfk_1` FOREIGN KEY (`menu_id`) REFERENCES `sys_menu` (`id`),
  ADD CONSTRAINT `sys_role_menu_ibfk_2` FOREIGN KEY (`role_id`) REFERENCES `sys_role` (`role_id`);

--
-- 限制表 `sys_user_dept`
--
ALTER TABLE `sys_user_dept`
  ADD CONSTRAINT `sys_user_dept_ibfk_1` FOREIGN KEY (`user_id`) REFERENCES `sys_user` (`id`),
  ADD CONSTRAINT `sys_user_dept_ibfk_2` FOREIGN KEY (`dept_id`) REFERENCES `sys_dept` (`dept_id`);

--
-- 限制表 `sys_user_post`
--
ALTER TABLE `sys_user_post`
  ADD CONSTRAINT `sys_user_post_ibfk_1` FOREIGN KEY (`user_id`) REFERENCES `sys_user` (`id`),
  ADD CONSTRAINT `sys_user_post_ibfk_2` FOREIGN KEY (`post_id`) REFERENCES `sys_post` (`post_id`);

--
-- 限制表 `sys_user_role`
--
ALTER TABLE `sys_user_role`
  ADD CONSTRAINT `sys_user_role_ibfk_1` FOREIGN KEY (`role_id`) REFERENCES `sys_role` (`role_id`),
  ADD CONSTRAINT `sys_user_role_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `sys_user` (`id`);
COMMIT;

/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
