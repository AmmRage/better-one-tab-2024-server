// 弹窗主逻辑
let isLoggedIn = false;

// DOM 元素
const loginBtn = document.getElementById('login-btn');
const uploadBtn = document.getElementById('upload-btn');
const downloadBtn = document.getElementById('download-btn');
const loginModal = document.getElementById('login-modal');
const loginForm = document.getElementById('login-form');
const cancelLoginBtn = document.getElementById('cancel-login');
const closeLoginModalBtn = document.getElementById('close-login-modal');
const groupsList = document.getElementById('groups-list');
const emptyMessage = document.getElementById('empty-message');
const loginError = document.getElementById('login-error');

// 初始化
document.addEventListener('DOMContentLoaded', async () => {
  await checkLoginStatus();
  await loadGroups();
  setupEventListeners();
});

// 设置事件监听
function setupEventListeners() {
  loginBtn.addEventListener('click', () => {
    showLoginModal();
  });

  cancelLoginBtn.addEventListener('click', () => {
    hideLoginModal();
  });

  closeLoginModalBtn.addEventListener('click', () => {
    hideLoginModal();
  });

  // 点击模态框外部关闭
  loginModal.addEventListener('click', (e) => {
    if (e.target === loginModal) {
      hideLoginModal();
    }
  });

  loginForm.addEventListener('submit', async (e) => {
    e.preventDefault();
    await handleLogin();
  });

  uploadBtn.addEventListener('click', async () => {
    await handleUpload();
  });

  downloadBtn.addEventListener('click', async () => {
    await handleDownload();
  });
}

// 检查登录状态
async function checkLoginStatus() {
  const config = await Storage.getServerConfig();
  if (config.serverUrl && config.username && config.token) {
    // 验证 token 是否有效
    const isValid = await API.verifyToken(config.serverUrl, config.username, config.token);
    if (isValid) {
      isLoggedIn = true;
      updateLoginUI();
    } else {
      // token 无效，清除配置
      await Storage.saveServerConfig('', '', '');
    }
  }
}

// 更新登录 UI
function updateLoginUI() {
  if (isLoggedIn) {
    loginBtn.textContent = '已登录';
    loginBtn.disabled = true;
    uploadBtn.disabled = false;
    downloadBtn.disabled = false;
  } else {
    loginBtn.textContent = '登录';
    loginBtn.disabled = false;
    uploadBtn.disabled = true;
    downloadBtn.disabled = true;
  }
}

// 显示登录模态框
async function showLoginModal() {
  const config = await Storage.getServerConfig();
  document.getElementById('server-url').value = config.serverUrl || '';
  document.getElementById('username').value = config.username || '';
  document.getElementById('password').value = '';
  loginError.classList.add('hidden');
  loginModal.classList.remove('hidden');
}

// 隐藏登录模态框
function hideLoginModal() {
  loginModal.classList.add('hidden');
}

// 处理登录
async function handleLogin() {
  const serverUrl = document.getElementById('server-url').value.trim();
  const username = document.getElementById('username').value.trim();
  const password = document.getElementById('password').value;

  if (!serverUrl || !username || !password) {
    showLoginError('请填写所有字段');
    return;
  }

  const result = await API.login(serverUrl, username, password);
  if (result.success) {
    isLoggedIn = true;
    updateLoginUI();
    hideLoginModal();
  } else {
    showLoginError(result.error || '登录失败');
  }
}

// 显示登录错误
function showLoginError(message) {
  loginError.textContent = message;
  loginError.classList.remove('hidden');
}

// 处理上传
async function handleUpload() {
  const config = await Storage.getServerConfig();
  const groups = await Storage.getAllGroups();

  uploadBtn.disabled = true;
  uploadBtn.textContent = '上传中...';

  const result = await API.uploadTabs(config.serverUrl, config.username, config.token, groups);
  
  uploadBtn.disabled = false;
  uploadBtn.textContent = '上传';

  if (result.success) {
    alert('上传成功');
  } else {
    alert('上传失败: ' + result.error);
  }
}

// 处理下载
async function handleDownload() {
  if (!confirm('下载将覆盖本地所有分组，确定继续吗？')) {
    return;
  }

  const config = await Storage.getServerConfig();

  downloadBtn.disabled = true;
  downloadBtn.textContent = '下载中...';

  const result = await API.downloadTabs(config.serverUrl, config.username, config.token);
  
  downloadBtn.disabled = false;
  downloadBtn.textContent = '下载';

  if (result.success) {
    await Storage.saveAllGroups(result.data);
    await loadGroups();
    alert('下载成功');
  } else {
    alert('下载失败: ' + result.error);
  }
}

// 加载分组列表
async function loadGroups() {
  const groups = await Storage.getAllGroups();
  
  if (groups.length === 0) {
    groupsList.innerHTML = '';
    emptyMessage.classList.remove('hidden');
    return;
  }

  emptyMessage.classList.add('hidden');
  groupsList.innerHTML = '';

  // 按时间倒序排列
  const sortedGroups = [...groups].sort((a, b) => b.time - a.time);

  sortedGroups.forEach(group => {
    const groupElement = createGroupElement(group);
    groupsList.appendChild(groupElement);
  });
}

// 创建分组元素
function createGroupElement(group) {
  const groupDiv = document.createElement('div');
  groupDiv.className = 'group-item';
  groupDiv.dataset.groupId = group._id;

  const timeStr = new Date(group.time).toLocaleString('zh-CN');

  groupDiv.innerHTML = `
    <div class="group-header">
      <div class="group-title">
        <span class="expand-icon ${group.expand ? 'expanded' : ''}">▶</span>
        ${group.titleEditing ? 
          `<input type="text" value="${group.title}" class="group-title-input">` :
          `<span class="group-title-text">${group.title}</span>`
        }
        <span class="group-time">${timeStr}</span>
      </div>
      <div class="group-actions">
        <button class="btn btn-danger delete-group-btn">删除分组</button>
      </div>
    </div>
    <div class="tabs-list ${group.expand ? '' : 'hidden'}">
      ${group.tabs.map(tab => `
        <div class="tab-item" data-tab-uuid="${tab.uuid}">
          <img src="${tab.favIconUrl || 'data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22><rect width=%2216%22 height=%2216%22 fill=%22%23999%22/></svg>'}" 
               class="tab-favicon">
          <div class="tab-info">
            <div class="tab-title">${escapeHtml(tab.title)}</div>
            <div class="tab-url">${escapeHtml(tab.url)}</div>
          </div>
          <div class="tab-actions">
            <a href="${tab.url}" target="_blank" class="tab-link">打开</a>
            <button class="btn btn-danger delete-tab-btn" style="padding: 2px 6px; font-size: 11px;">删除</button>
          </div>
        </div>
      `).join('')}
    </div>
  `;

  // 绑定事件
  const header = groupDiv.querySelector('.group-header');
  const expandIcon = groupDiv.querySelector('.expand-icon');
  const deleteGroupBtn = groupDiv.querySelector('.delete-group-btn');
  const titleText = groupDiv.querySelector('.group-title-text');
  const titleInput = groupDiv.querySelector('.group-title-input');
  const tabsList = groupDiv.querySelector('.tabs-list');
  const deleteTabBtns = groupDiv.querySelectorAll('.delete-tab-btn');

  // 展开/折叠
  header.addEventListener('click', async (e) => {
    if (e.target === deleteGroupBtn || e.target.closest('.delete-group-btn')) {
      return;
    }
    if (e.target === titleInput || e.target.closest('.group-title-input')) {
      return;
    }
    
    group.expand = !group.expand;
    expandIcon.classList.toggle('expanded');
    tabsList.classList.toggle('hidden');
    await Storage.toggleGroupExpand(group._id);
  });

  // 编辑标题
  if (titleText) {
    titleText.addEventListener('dblclick', () => {
      titleText.style.display = 'none';
      const input = document.createElement('input');
      input.type = 'text';
      input.value = group.title;
      input.className = 'group-title-input';
      titleText.parentNode.insertBefore(input, titleText);
      input.focus();
      input.select();

      const saveTitle = async () => {
        const newTitle = input.value.trim() || group.title;
        titleText.textContent = newTitle;
        input.remove();
        titleText.style.display = '';
        await Storage.updateGroupTitle(group._id, newTitle);
        await loadGroups();
      };

      input.addEventListener('blur', saveTitle);
      input.addEventListener('keypress', (e) => {
        if (e.key === 'Enter') {
          saveTitle();
        }
      });
    });
  }

  if (titleInput) {
    titleInput.addEventListener('blur', async () => {
      const newTitle = titleInput.value.trim() || group.title;
      await Storage.updateGroupTitle(group._id, newTitle);
      await loadGroups();
    });
    titleInput.addEventListener('keypress', async (e) => {
      if (e.key === 'Enter') {
        titleInput.blur();
      }
    });
  }

  // 删除分组
  deleteGroupBtn.addEventListener('click', async (e) => {
    e.stopPropagation();
    if (confirm('确定要删除这个分组吗？')) {
      await Storage.deleteGroup(group._id);
      await loadGroups();
    }
  });

  // 删除标签
  deleteTabBtns.forEach(btn => {
    btn.addEventListener('click', async (e) => {
      e.stopPropagation();
      const tabItem = btn.closest('.tab-item');
      const tabUuid = tabItem.dataset.tabUuid;
      if (confirm('确定要删除这个标签吗？')) {
        await Storage.deleteTab(group._id, tabUuid);
        await loadGroups();
      }
    });
  });

  // 处理 favicon 加载失败
  const favicons = groupDiv.querySelectorAll('.tab-favicon');
  const fallbackFavicon = 'data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg"><rect width="16" height="16" fill="#999"/></svg>';
  favicons.forEach(img => {
    img.addEventListener('error', function() {
      this.src = fallbackFavicon;
    });
  });

  return groupDiv;
}

// HTML 转义
function escapeHtml(text) {
  const div = document.createElement('div');
  div.textContent = text;
  return div.innerHTML;
}
