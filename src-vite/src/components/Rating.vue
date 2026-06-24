<template>
  <div class="sidebar-panel">
    <div class="sidebar-panel-header">
      <span class="sidebar-panel-header-title flex-1">{{ titlebar || $t('rating.title') }}</span>
    </div>

    <ul class="flex-1 overflow-x-hidden overflow-y-auto">
      <li>
        <div
          :class="[
            'sidebar-item',
            libConfig.rating.item === 0 ? 'sidebar-item-selected' : 'sidebar-item-hover',
          ]"
          @click="clickRating(0)"
        >
          <div class="mx-1 flex items-center gap-2">
            <IconStar class="w-5 h-5 shrink-0" />
            <span>{{ $t('rating.unrated') }}</span>
          </div>
          <span v-if="unratedCount" class="sidebar-item-count ml-auto">{{ unratedCount.toLocaleString() }}</span>
        </div>
      </li>
      <li v-for="rating in [5, 4, 3, 2, 1]" :key="rating">
        <div
          :class="[
            'sidebar-item',
            libConfig.rating.item === rating ? 'sidebar-item-selected' : 'sidebar-item-hover',
          ]"
          @click="clickRating(rating)"
        >
          <div class="mx-1 flex items-center gap-0.5">
            <IconStarFilled
              v-for="index in rating"
              :key="index"
              class="w-5 h-5 shrink-0"
            />
          </div>
          <span v-if="ratingCounts[rating]" class="sidebar-item-count ml-auto">{{ ratingCounts[rating].toLocaleString() }}</span>
        </div>
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { libConfig } from '@/common/config';
import { getQueryCountAndSum } from '@/common/api';
import { IconStar, IconStarFilled } from '@/common/icons';

defineProps<{
  titlebar?: string;
}>();

const unratedCount = ref(0);
const ratingCounts = ref<Record<number, number>>({
  1: 0,
  2: 0,
  3: 0,
  4: 0,
  5: 0,
});

const buildQueryParams = ({ rating = -1 } = {}) => ({
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
  isFavorite: false,
  rating,
  tagId: 0,
  personId: 0,
});

async function loadCounts() {
  const unrated = await getQueryCountAndSum(buildQueryParams({ rating: 0 }));
  unratedCount.value = unrated ? Number(unrated[0]) : 0;

  const entries = await Promise.all(
    [1, 2, 3, 4, 5].map(async (rating) => {
      const result = await getQueryCountAndSum(buildQueryParams({ rating }));
      return [rating, result ? Number(result[0]) : 0] as const;
    }),
  );

  ratingCounts.value = Object.fromEntries(entries) as Record<number, number>;
}

function clickRating(rating: number) {
  libConfig.rating.item = rating;
}

onMounted(() => {
  if (typeof libConfig.rating.item !== 'number') {
    libConfig.rating.item = 0;
  }
  void loadCounts();
});
</script>
