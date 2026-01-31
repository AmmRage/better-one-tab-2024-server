// 存储管理工具
const Storage = {
  // 生成唯一 ID
  generateId() {
    return Date.now().toString(36) + Math.random().toString(36).substr(2);
  },

  // 获取所有分组
  async getAllGroups() {
    const result = await chrome.storage.local.get(['tabGroups']);
    return result.tabGroups || [];
  },

  // 保存所有分组
  async saveAllGroups(groups) {
    await chrome.storage.local.set({ tabGroups: groups });
  },

  // 添加新分组
  async addGroup(tabs) {
    const groups = await this.getAllGroups();
    const group = {
      _id: this.generateId(),
      uuid: this.generateId(),
      color: '#4285f4',
      expand: false,
      pinned: false,
      tabs: tabs,
      tags: [],
      time: Date.now(),
      title: new Date().toLocaleString('zh-CN'),
      titleEditing: false,
      updatedAt: Date.now()
    };
    groups.unshift(group);
    await this.saveAllGroups(groups);
    return group;
  },

  // 删除分组
  async deleteGroup(groupId) {
    const groups = await this.getAllGroups();
    const filtered = groups.filter(g => g._id !== groupId);
    await this.saveAllGroups(filtered);
  },

  // 删除标签
  async deleteTab(groupId, tabUuid) {
    const groups = await this.getAllGroups();
    const group = groups.find(g => g._id === groupId);
    if (group) {
      group.tabs = group.tabs.filter(t => t.uuid !== tabUuid);
      group.updatedAt = Date.now();
      await this.saveAllGroups(groups);
    }
  },

  // 更新分组标题
  async updateGroupTitle(groupId, title) {
    const groups = await this.getAllGroups();
    const group = groups.find(g => g._id === groupId);
    if (group) {
      group.title = title;
      group.updatedAt = Date.now();
      await this.saveAllGroups(groups);
    }
  },

  // 切换分组展开/折叠
  async toggleGroupExpand(groupId) {
    const groups = await this.getAllGroups();
    const group = groups.find(g => g._id === groupId);
    if (group) {
      group.expand = !group.expand;
      await this.saveAllGroups(groups);
    }
  },

  // 获取服务器配置
  async getServerConfig() {
    const result = await chrome.storage.local.get(['serverUrl', 'username', 'token']);
    // 确保 token 是纯字符串，去除可能的引号
    let token = result.token || '';
    if (typeof token === 'string') {
      token = token.replace(/^["']|["']$/g, '').trim();
    }
    return {
      serverUrl: result.serverUrl || '',
      username: result.username || '',
      token: token
    };
  },

  // 保存服务器配置
  async saveServerConfig(serverUrl, username, token) {
    await chrome.storage.local.set({
      serverUrl: serverUrl,
      username: username,
      token: token
    });
  }
};
