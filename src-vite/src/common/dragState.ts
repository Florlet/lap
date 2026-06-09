import { reactive } from 'vue';

export interface DragFile {
  id: number;
  file_path: string;
  folder_id: number;
}

export const dragState = reactive<{
  files: DragFile[] | null;
}>({
  files: null,
});
