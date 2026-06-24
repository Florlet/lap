<template>

  <div class="sidebar-panel">
    <div class="sidebar-panel-header">
      <span class="sidebar-panel-header-title flex-1">{{ titlebar }}</span>
    </div>

    <div
      v-for="item in libraryItems"
      :key="item.id"
      :class="[
        'sidebar-item mb-2',
        libConfig.library.item === item.id ? 'sidebar-item-selected' : 'sidebar-item-hover',
      ]"
      @click="selectItem(item.id)"
    >
      <component :is="item.icon" class="mx-1 w-5 h-5 shrink-0" />
      <div class="sidebar-item-label">
        <span>{{ item.label }}</span>
      </div>
      <div class="ml-auto flex items-center">
        <span v-if="item.count && item.count > 0" class="px-1 text-xs text-base-content/30">
          {{ item.count.toLocaleString() }}
        </span>
      </div>
    </div>

    <div class="sidebar-panel-header cursor-pointer" @click="toggleSubjects">
      <span class="sidebar-panel-header-title flex-1 min-w-0 overflow-hidden text-ellipsis whitespace-nowrap">
        {{ localeMsg.subject.title }}
      </span>
      <TButton
        :icon="libConfig.library.subjectsExpanded ? IconArrowDown : IconArrowUp"
        :buttonSize="'small'"
        @click.stop="toggleSubjects"
      />
    </div>

    <ul v-if="libConfig.library.subjectsExpanded" class="mb-2">
      <li v-for="item in smartTagItems" :key="item.id">
        <div
          :class="[
            'sidebar-item group border-2 border-transparent',
            libConfig.library.item === LIB_ITEM.SUBJECTS && libConfig.library.smartId === item.id ? 'sidebar-item-selected' : 'sidebar-item-hover',
          ]"
          @click="selectSmartTag(item.id)"
        >
          <IconSmartTag class="mx-1 w-5 h-5 shrink-0" />
          <span class="sidebar-item-label">{{ item.label }}</span>
        </div>
      </li>
    </ul>

    <CollectionTray
      v-if="config.settings.showCollections"
      class="overflow-hidden"
      :class="config.collectionTray.expanded ? '' : 'h-10'"
      :expanded="config.collectionTray.expanded"
      @toggle-expanded="toggleCollectionTray"
    />

  </div>

</template>

<script setup lang="ts">
import { computed, ref, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { config, libConfig } from '@/common/config';
import { LIB_ITEM, type LibItem } from '@/common/constants';

import { IconPhotoAll, IconHeart, IconCalendarDay, IconHistory, IconArrowDown, IconArrowUp, IconSmartTag } from '@/common/icons';
import { getQueryCountAndSum, getTotalCountAndSum } from '@/common/api';
import { SMART_TAG_CATEGORIES } from '@/common/smartTags';
import CollectionTray from '@/components/CollectionTray.vue';
import TButton from '@/components/TButton.vue';

const props = defineProps({
  titlebar: {
    type: String,
    required: true
  }
});

const { locale, messages } = useI18n();
const localeMsg = computed(() => messages.value[locale.value] as any);
const totalCount = ref(0);
const favoriteCount = ref(0);

const buildQueryParams = ({ isFavorite = false, rating = -1 } = {}) => ({
  searchFileType: 0,
  sortType: 0,
  sortOrder: 0,
  searchFileName: "",
  searchAllSubfolders: "",
  searchFolder: "",
  startDate: 0,
  endDate: 0,
  calendarSort: 0,
  make: "",
  model: "",
  lensMake: "",
  lensModel: "",
  locationAdmin1: "",
  locationName: "",
  isFavorite,
  rating,
  tagId: 0,
  personId: 0,
});

const libraryItems = computed(() => [
  {
    id: LIB_ITEM.ALL,
    label: localeMsg.value.library.all_files,
    icon: IconPhotoAll,
    count: totalCount.value,
  },
  {
    id: LIB_ITEM.FAV,
    label: localeMsg.value.favorite.files,
    icon: IconHeart,
    count: favoriteCount.value,
  },
  {
    id: LIB_ITEM.TODAY,
    label: localeMsg.value.library.on_this_day,
    icon: IconCalendarDay,
    count: 0,
  },
  {
    id: LIB_ITEM.RECENT,
    label: localeMsg.value.library.recently_added,
    icon: IconHistory,
    count: 0,
  },
]);

const smartTagItems = computed(() =>
  SMART_TAG_CATEGORIES.map(category => {
    const item = category.items[0];
    return {
      id: item.id,
      label: localeMsg.value.subject.items?.[item.id] || item.id,
    };
  })
);

const refreshTotalCount = async () => {
  const result = await getTotalCountAndSum();
  totalCount.value = result ? result[0] : 0;
};

const refreshFavoriteCount = async () => {
  const result = await getQueryCountAndSum(buildQueryParams({ isFavorite: true }));
  favoriteCount.value = result ? Number(result[0]) : 0;
};

function selectItem(item: LibItem) {
  libConfig.library.item = item;
}

function toggleSubjects() {
  libConfig.library.item = LIB_ITEM.SUBJECTS;
  libConfig.library.subjectsExpanded = !libConfig.library.subjectsExpanded;
}

function selectSmartTag(smartId: string) {
  libConfig.library.item = LIB_ITEM.SUBJECTS;
  libConfig.library.smartId = smartId;
}

function toggleCollectionTray() {
  config.collectionTray.expanded = !config.collectionTray.expanded;
}

onMounted(async () => {
  await refreshTotalCount();
  await refreshFavoriteCount();
});

</script>
