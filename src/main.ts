import { createApp } from 'vue';
import { createPinia } from 'pinia';
import ElementPlus from 'element-plus';
import 'element-plus/dist/index.css';
import 'element-plus/theme-chalk/dark/css-vars.css';
import zhCn from 'element-plus/es/locale/lang/zh-cn';
import 'md-editor-v3/lib/style.css';
import App from './App.vue';

// 初始化主题，避免刷新时闪烁
const savedTheme = localStorage.getItem('work-item-theme');
if (savedTheme === 'dark') {
  document.documentElement.classList.add('dark');
}

const app = createApp(App);
app.use(createPinia());
app.use(ElementPlus, { locale: zhCn });
app.mount('#app');
