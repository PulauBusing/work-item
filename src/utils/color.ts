// 根据背景色（hex）判断用深色还是白色文字，保证标签文字可读
export function textColor(hex: string): string {
  const c = hex.replace('#', '');
  if (c.length < 6) return '#303133';
  const r = parseInt(c.substring(0, 2), 16);
  const g = parseInt(c.substring(2, 4), 16);
  const b = parseInt(c.substring(4, 6), 16);
  const luminance = 0.299 * r + 0.587 * g + 0.114 * b;
  return luminance > 150 ? '#303133' : '#ffffff';
}

// 标签可选 chip 的样式：选中时高亮（全色背景），未选中时浅色背景 + 原色文字
export function chipStyle(color: string, active: boolean): Record<string, string> {
  if (active) {
    return { backgroundColor: color, color: textColor(color) };
  }
  return { backgroundColor: color + '2E', color: color };
}
