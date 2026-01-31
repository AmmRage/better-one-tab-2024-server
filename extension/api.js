// API 调用工具
const API = {
  // 获取服务器配置
  async getConfig() {
    return await Storage.getServerConfig();
  },

  // 登录
  async login(serverUrl, username, password) {
    try {
      const response = await fetch(`${serverUrl}/api/verify`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ username, password })
      });

      if (response.ok) {
        const token = (await response.text()).trim();
        await Storage.saveServerConfig(serverUrl, username, token);
        return { success: true, token };
      } else {
        const error = await response.text();
        return { success: false, error };
      }
    } catch (error) {
      return { success: false, error: error.message };
    }
  },

  // 上传标签
  async uploadTabs(serverUrl, username, token, groups) {
    try {
      // 确保 token 是纯字符串，去除可能的引号
      const cleanToken = String(token).replace(/^["']|["']$/g, '').trim();
      const response = await fetch(`${serverUrl}/api/user/${username}/tabs`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          tabs: groups,
          token: cleanToken
        })
      });

      if (response.ok) {
        const result = await response.json();
        return { success: true, data: result };
      } else {
        const error = await response.text();
        return { success: false, error };
      }
    } catch (error) {
      return { success: false, error: error.message };
    }
  },

  // 下载标签
  async downloadTabs(serverUrl, username, token) {
    try {
      // 确保 token 是纯字符串，去除可能的引号，并正确编码
      const cleanToken = String(token).replace(/^["']|["']$/g, '').trim();
      const response = await fetch(`${serverUrl}/api/user/${username}/tabs?token=${encodeURIComponent(cleanToken)}`, {
        method: 'GET'
      });

      if (response.ok) {
        const result = await response.json();
        return { success: true, data: result.tabs || [] };
      } else {
        const error = await response.text();
        return { success: false, error };
      }
    } catch (error) {
      return { success: false, error: error.message };
    }
  },

  // 验证 token
  async verifyToken(serverUrl, username, token) {
    try {
      // 确保 token 是纯字符串，去除可能的引号，并正确编码
      const cleanToken = String(token).replace(/^["']|["']$/g, '').trim();
      const response = await fetch(`${serverUrl}/api/user/${username}?token=${encodeURIComponent(cleanToken)}`, {
        method: 'GET'
      });
      return response.ok;
    } catch (error) {
      return false;
    }
  },

  // 退出登录
  async logout(serverUrl, username, token) {
    try {
      // 确保 token 是纯字符串，去除可能的引号
      const cleanToken = String(token).replace(/^["']|["']$/g, '').trim();
      const response = await fetch(`${serverUrl}/api/user/${username}/logout`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(cleanToken)
      });

      if (response.ok) {
        return { success: true };
      } else {
        const error = await response.text();
        return { success: false, error };
      }
    } catch (error) {
      return { success: false, error: error.message };
    }
  }
};
