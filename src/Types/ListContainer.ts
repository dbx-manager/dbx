// this name is misleading , this name is the same that is used in the backend (podman-api) that represent a single contaienr 
// and i keept it the same so no confusion is happening betwean the type and the name betwean the backend and the frontend
export interface ListContainer {
  AutoRemove: boolean;
  Command: string[];
  Created: string;          
  CreatedAt: string;        
  ExitCode: number;
  Exited: boolean;
  ExitedAt: number;         
  Id: string;
  Image: string;
  ImageID: string;
  IsInfra: boolean;
  Labels: Record<string, string>; 
  Mounts: string[];
  Names: string[];
  Namespaces: Record<string, unknown>; 
  Networks: unknown[];      
  Pid: number;
  Pod: string;
  PodName: string;
  Size: number | null;      
  StartedAt: number;        
  State: string;            
  Status: string;           
}