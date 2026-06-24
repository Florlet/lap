// sidebar index enum
export const SIDEBAR = {
  LIBRARY: 0,
  ALBUM: 1,
  SMART_ALBUM: 2,
  SEARCH: 3,
  RATING: 4,
  TAG: 5,
  CALENDAR: 6,
  PERSON: 7,
  LOCATION: 8,
  CAMERA: 9,
  MAP: 10,
} as const;

export type Sidebar = (typeof SIDEBAR)[keyof typeof SIDEBAR];

// library panel item enum
export const LIB_ITEM = {
  ALL: 'all-files',
  FAV: 'favorites',
  TODAY: 'on-this-day',
  RECENT: 'recently-added',
  SUBJECTS: 'subjects',
} as const;

export type LibItem = (typeof LIB_ITEM)[keyof typeof LIB_ITEM];

// group by enum
export const GROUP = {
  NONE: 0,
  FOLDER: 1,
  DAY: 2,
  MONTH: 3,
  RATING: 4,
  LOCATION: 5,
  CAMERA: 6,
  LENS: 7,
} as const;

export type Group = (typeof GROUP)[keyof typeof GROUP];
