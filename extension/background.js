// 后台服务
chrome.runtime.onInstalled.addListener(() => {
  // 创建右键菜单
  chrome.contextMenus.create({
    id: 'save-current-tab',
    title: '保存并保存当前标签',
    contexts: ['page']
  });

  chrome.contextMenus.create({
    id: 'save-window-tabs',
    title: '保存并关闭当前窗口所有标签',
    contexts: ['page']
  });

  chrome.contextMenus.create({
    id: 'save-all-tabs',
    title: '保存并关闭所有窗口标签',
    contexts: ['page']
  });
});

// 处理扩展图标点击
chrome.action.onClicked.addListener(async (tab) => {
  // 打开新标签页显示扩展页面
  await chrome.tabs.create({
    url: chrome.runtime.getURL('popup.html')
  });
});

// 处理右键菜单点击
chrome.contextMenus.onClicked.addListener(async (info, tab) => {
  if (info.menuItemId === 'save-current-tab') {
    await saveCurrentTab(tab);
  } else if (info.menuItemId === 'save-window-tabs') {
    await saveWindowTabs(tab.windowId);
  } else if (info.menuItemId === 'save-all-tabs') {
    await saveAllTabs();
  }
});

// 生成唯一 ID
function generateId() {
  return Date.now().toString(36) + Math.random().toString(36).substr(2);
}

// 获取所有分组
async function getAllGroups() {
  const result = await chrome.storage.local.get(['tabGroups']);
  return result.tabGroups || [];
}

// 保存所有分组
async function saveAllGroups(groups) {
  await chrome.storage.local.set({ tabGroups: groups });
}

// 保存当前标签
async function saveCurrentTab(tab) {
  const tabs = [{
    uuid: generateId(),
    favIconUrl: tab.favIconUrl || '',
    muted: tab.mutedInfo?.muted || false,
    pinned: tab.pinned || false,
    title: tab.title || '',
    url: tab.url || ''
  }];

  const groups = await getAllGroups();
  const group = {
    _id: generateId(),
    uuid: generateId(),
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
  await saveAllGroups(groups);

  // 关闭当前标签
  chrome.tabs.remove(tab.id);
}

// 保存当前窗口所有标签
async function saveWindowTabs(windowId) {
  const tabs = await chrome.tabs.query({ windowId: windowId });
  const tabData = tabs.map(tab => ({
    uuid: generateId(),
    favIconUrl: tab.favIconUrl || '',
    muted: tab.mutedInfo?.muted || false,
    pinned: tab.pinned || false,
    title: tab.title || '',
    url: tab.url || ''
  }));

  const groups = await getAllGroups();
  const group = {
    _id: generateId(),
    uuid: generateId(),
    color: '#4285f4',
    expand: false,
    pinned: false,
    tabs: tabData,
    tags: [],
    time: Date.now(),
    title: new Date().toLocaleString('zh-CN'),
    titleEditing: false,
    updatedAt: Date.now()
  };
  groups.unshift(group);
  await saveAllGroups(groups);

  // 先打开扩展页面
  const extensionUrl = chrome.runtime.getURL('popup.html');
  const extensionTab = await chrome.tabs.create({
    url: extensionUrl,
    windowId: windowId
  });

  // 关闭其他标签（排除扩展页面）
  const tabIds = tabs
    .filter(t => t.url !== extensionUrl)
    .map(t => t.id);
  
  if (tabIds.length > 0) {
    chrome.tabs.remove(tabIds);
  }
}

// 保存所有窗口所有标签
async function saveAllTabs() {
  const tabs = await chrome.tabs.query({});
  const extensionUrl = chrome.runtime.getURL('popup.html');
  
  const tabData = tabs
    .filter(t => t.url !== extensionUrl) // 排除已存在的扩展页面
    .map(tab => ({
      uuid: generateId(),
      favIconUrl: tab.favIconUrl || '',
      muted: tab.mutedInfo?.muted || false,
      pinned: tab.pinned || false,
      title: tab.title || '',
      url: tab.url || ''
    }));

  const groups = await getAllGroups();
  const group = {
    _id: generateId(),
    uuid: generateId(),
    color: '#4285f4',
    expand: false,
    pinned: false,
    tabs: tabData,
    tags: [],
    time: Date.now(),
    title: new Date().toLocaleString('zh-CN'),
    titleEditing: false,
    updatedAt: Date.now()
  };
  groups.unshift(group);
  await saveAllGroups(groups);

  // 先打开扩展页面（在第一个窗口中）
  const windows = await chrome.windows.getAll();
  const firstWindowId = windows.length > 0 ? windows[0].id : null;
  
  await chrome.tabs.create({
    url: extensionUrl,
    windowId: firstWindowId
  });

  // 关闭所有其他标签（排除扩展页面）
  const tabIds = tabs
    .filter(t => t.url !== extensionUrl)
    .map(t => t.id);
  
  if (tabIds.length > 0) {
    chrome.tabs.remove(tabIds);
  }
}
