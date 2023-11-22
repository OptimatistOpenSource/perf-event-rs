pub enum RecordBody {
    /*
    struct {
      u32    pid, tid;
      u64    addr;
      u64    len;
      u64    pgoff;
      char   filename[];
    };
    */
    Mmap,

    /*
    struct {
      u64    id;
      u64    lost;
      struct sample_id sample_id;
    };
    */
    Lost,

    /*
    struct {
      u32    pid;
      u32    tid;
      char   comm[];
      struct sample_id sample_id;
    };
    */
    Comm,

    /*
    struct {
      u32    pid, ppid;
      u32    tid, ptid;
      u64    time;
      struct sample_id sample_id;
    };
    */
    Exit,

    /*
    struct {
      u64    time;
      u64    id;
      u64    stream_id;
      struct sample_id sample_id;
    };
    */
    Throttle,

    /*
    struct {
      u64    time;
      u64    id;
      u64    stream_id;
      struct sample_id sample_id;
    };
    */
    Unthrottle,

    /*
    struct {
      u32    pid, ppid;
      u32    tid, ptid;
      u64    time;
      struct sample_id sample_id;
    };
    */
    Fork,

    /*
    struct {
      u32    pid, tid;
      struct read_format values;
      struct sample_id sample_id;
    };
    */
    Read,

    /*
    struct {
      u64    sample_id;   /* if PERF_SAMPLE_IDENTIFIER */
      u64    ip;          /* if PERF_SAMPLE_IP */
      u32    pid, tid;    /* if PERF_SAMPLE_TID */
      u64    time;        /* if PERF_SAMPLE_TIME */
      u64    addr;        /* if PERF_SAMPLE_ADDR */
      u64    id;          /* if PERF_SAMPLE_ID */
      u64    stream_id;   /* if PERF_SAMPLE_STREAM_ID */
      u32    cpu, res;    /* if PERF_SAMPLE_CPU */
      u64    period;      /* if PERF_SAMPLE_PERIOD */
      struct read_format v;
                          /* if PERF_SAMPLE_READ */
      u64    nr;          /* if PERF_SAMPLE_CALLCHAIN */
      u64    ips[nr];     /* if PERF_SAMPLE_CALLCHAIN */
      u32    size;        /* if PERF_SAMPLE_RAW */
      char   data[size];  /* if PERF_SAMPLE_RAW */
      u64    bnr;         /* if PERF_SAMPLE_BRANCH_STACK */
      struct perf_branch_entry lbr[bnr];
                          /* if PERF_SAMPLE_BRANCH_STACK */
      u64    abi;         /* if PERF_SAMPLE_REGS_USER */
      u64    regs[weight(mask)];
                          /* if PERF_SAMPLE_REGS_USER */
      u64    size;        /* if PERF_SAMPLE_STACK_USER */
      char   data[size];  /* if PERF_SAMPLE_STACK_USER */
      u64    dyn_size;    /* if PERF_SAMPLE_STACK_USER &&
                             size != 0 */
      union perf_sample_weight weight;
                          /* if PERF_SAMPLE_WEIGHT */
                          /* || PERF_SAMPLE_WEIGHT_STRUCT */
      u64    data_src;    /* if PERF_SAMPLE_DATA_SRC */
      u64    transaction; /* if PERF_SAMPLE_TRANSACTION */
      u64    abi;         /* if PERF_SAMPLE_REGS_INTR */
      u64    regs[weight(mask)];
                          /* if PERF_SAMPLE_REGS_INTR */
      u64    phys_addr;   /* if PERF_SAMPLE_PHYS_ADDR */
      u64    cgroup;      /* if PERF_SAMPLE_CGROUP */
      u64    data_page_size;
                        /* if PERF_SAMPLE_DATA_PAGE_SIZE */
      u64    code_page_size;
                        /* if PERF_SAMPLE_CODE_PAGE_SIZE */
      u64    size;        /* if PERF_SAMPLE_AUX */
      char   data[size];  /* if PERF_SAMPLE_AUX */
    };
    */
    Sample,

    /*
    struct {
      u32    pid;
      u32    tid;
      u64    addr;
      u64    len;
      u64    pgoff;
      union {
          struct {
              u32    maj;
              u32    min;
              u64    ino;
              u64    ino_generation;
          };
          struct {   /* if PERF_RECORD_MISC_MMAP_BUILD_ID */
              u8     build_id_size;
              u8     __reserved_1;
              u16    __reserved_2;
              u8     build_id[20];
          };
      };
      u32    prot;
      u32    flags;
      char   filename[];
      struct sample_id sample_id;
    };
    */
    Mmap2,

    /*
    struct {
      u64    aux_offset;
      u64    aux_size;
      u64    flags;
      struct sample_id sample_id;
    };
    */
    Aux,

    /*
    struct {
      u32    pid;
      u32    tid;
    };
    */
    ItraceStart,

    /*
    struct {
      u64    lost;
      struct sample_id sample_id;
    };
    */
    LostSamples,

    /*
    struct {
      struct sample_id sample_id;
    };
    */
    Switch,

    /*
    struct {
      u32 next_prev_pid;
      u32 next_prev_tid;
      struct sample_id sample_id;
    };
    */
    SwitchCpuWide,

    /*
    struct {
      u32    pid;
      u32    tid;
      u64    nr_namespaces;
      struct { u64 dev, inode } [nr_namespaces];
      struct sample_id sample_id;
    };
    */
    Namespaces,

    /*
    struct {
      u64    addr;
      u32    len;
      u16    ksym_type;
      u16    flags;
      char   name[];
      struct sample_id sample_id;
    };
    */
    Ksymbol,

    /*
    struct {
      u16 type;
      u16 flags;
      u32 id;
      u8 tag[BPF_TAG_SIZE];
      struct sample_id sample_id;
    };
    */
    BpfEvent,

    /*
    struct {
      u64    id;
      char   path[];
      struct sample_id sample_id;
    };
    */
    Cgroup,

    /*
    struct {
      u64    addr;
      u16    old_len;
      u16    new_len;
      u8     bytes[];
      struct sample_id sample_id;
    };
    */
    TextPoke,

    AuxOutputHwId, // TODO: missing docs in man
}
