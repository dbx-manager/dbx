interface BackupFileList{
    files:BackupFile[];
    path:string;
    total_size_bytes:number
}
interface BackupFile{
    name:string;
    path:string;
    size_bytes:number;
    is_dir:boolean;
}