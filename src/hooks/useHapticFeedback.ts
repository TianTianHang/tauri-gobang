const HAPTIC = {
  PLACE: 10,
  WIN: [20, 50, 20] as [number, number, number],
  MENU_OPEN: 5,
  ERROR: 50,
};

export function useHapticFeedback() {
  const vibrate = (pattern: number | number[]) => {
    if (typeof navigator !== "undefined" && "vibrate" in navigator) {
      navigator.vibrate(pattern);
    }
  };

  return {
    place: () => vibrate(HAPTIC.PLACE),
    win: () => vibrate(HAPTIC.WIN),
    menuOpen: () => vibrate(HAPTIC.MENU_OPEN),
    error: () => vibrate(HAPTIC.ERROR),
  };
}
