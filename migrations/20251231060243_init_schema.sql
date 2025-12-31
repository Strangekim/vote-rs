-- 1. 사용자 테이블 (생성자 및 투표자)
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 2. 안건(Agenda) 테이블
-- 요구사항: 생성자, 안건 이름(내용), 생성일, 찬반 캐싱
CREATE TABLE agendas (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,           -- 안건 이름 (내용 포함)
    created_by UUID NOT NULL REFERENCES users(id), -- 생성자 (FK)
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- [성능 최적화 핵심] 반정규화(Denormalization) 컬럼
    -- votes 테이블을 매번 count() 하는 비용을 줄이기 위한 캐싱 컬럼
    agree_count INT NOT NULL DEFAULT 0,
    disagree_count INT NOT NULL DEFAULT 0
);

-- 3. 투표 내역 (Votes)
-- 요구사항: 안건 FK, 사용자 1명은 찬/반 중 하나만 선택 가능
CREATE TABLE votes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),    -- 누가
    agenda_id UUID NOT NULL REFERENCES agendas(id), -- 어떤 안건에
    
    -- 찬성(true) / 반대(false)
    -- 확장성을 위해 TEXT로 할 수도 있지만, 성능(용량)상 BOOLEAN이 가장 가볍습니다.
    is_agree BOOLEAN NOT NULL, 
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- [핵심 제약 조건]
    -- "한 사용자는 하나의 안건에 대해 한 번만 투표 가능"
    -- (user_id + agenda_id) 조합은 유일해야 함
    CONSTRAINT uk_vote_user_agenda UNIQUE (user_id, agenda_id)
);

-- 인덱스 설정 (조회 성능 향상용)
CREATE INDEX idx_votes_agenda ON votes(agenda_id); -- 안건별 투표 내역 조회용
CREATE INDEX idx_agendas_created_by ON agendas(created_by); -- 내가 만든 안건 조회용