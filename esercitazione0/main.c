#include <stdio.h>
#include <stdlib.h>
#include <string.h>


typedef struct {
    int type;//4
    float val;//4
    long timestamp;//8
} ValueStruct;
typedef struct {
    int type;//4
    float val[10];//40+nullterminated(4)
    long timestamp;//8
} MValueStruct;
typedef struct {
    int type;//4
    char message[21]; // stringa null terminated lung max 20 //21
} MessageStruct;
typedef struct {
    int type;//4
    union {//+4 null
    ValueStruct val;//20
    MValueStruct mvals;//56
    MessageStruct messages;//28
    };
} ExportData;

void export(ExportData *data, int n, FILE *pf) {
    fwrite(data, sizeof(ExportData),n,pf);
}

int main(int argc, char **argv){
    ExportData V[100];
    int i,j,k;
    unsigned char* byte;
    ExportData data;
    FILE* f;
    for(i=0;i<100;i++){
        j=i%3;
        if(j==0){
            data.type=1;
            data.val.type=1;
            j=j%2;
            if(j==0){
                data.val.val=1.0f;
            }
            else{
                data.val.val=2.0f;
            }
            j=j%4;
            if(j==0){
                data.val.timestamp=50000;
            }
            else if(j==1){
                data.val.timestamp=60000;
            }
            else if(j==2){
                data.val.timestamp=70000;
            }
            else{
                data.val.timestamp=80000;
            }
        }
        else if(j==1){
            data.type=2;
            data.val.type=2;
            j=j%2;
            if(j==0){
                for (k=0;k<10;k++) {
                    data.mvals.val[k]=0.0;
                }
            }
            else{
                for (k=0;k<10;k++) {
                    data.mvals.val[k]=1.0;
                }
            }
            j=j%3;
            if(j==0){
                data.mvals.timestamp=25000;
            }
            else if(j==1){
                data.mvals.timestamp=30000;
            }
            else{
                data.mvals.timestamp=35000;
            }
        }
        else{
            data.type=3;
            data.messages.type=3;
            j=0;
            if(j==0){
                strcpy(data.messages.message, "config");
            }
            else if(j==1){
                strcpy(data.messages.message, "code");
            }
            else{
                strcpy(data.messages.message, "memory");
            }
        }
        V[i]=data;
        
        if(i<5)//per visualizzazione solo i primi 5
            for(j=0;j<sizeof(ExportData);j++){
                printf("%d\n",j);
                byte=((unsigned char*)(&V[i]))+j;
                for(k=7;k>=0;k--){
                    if(((*byte)&(1<<k))==0)
                    {
                        printf("0");
                    }
                    else if(((*byte)&(1<<k))==(1<<k))
                    {
                        printf("1");
                    }
                }
                printf("\n");
            }
        for(j=0;j<sizeof(ExportData);j++){
            *(((unsigned char*)(&data))+j)=0;//bring to 0 unused area
        }
    }
    f=fopen("data","w");
    export(V,100,f);
    return 0;
}